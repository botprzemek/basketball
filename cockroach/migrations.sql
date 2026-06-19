DROP DATABASE IF EXISTS dev CASCADE;

CREATE DATABASE IF NOT EXISTS dev;

USE dev;

CREATE SCHEMA IF NOT EXISTS auth;

DROP USER IF EXISTS dev;

CREATE USER IF NOT EXISTS dev WITH PASSWORD NULL; --'your-password';

GRANT USAGE ON SCHEMA auth TO dev;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA auth TO dev;
ALTER DEFAULT PRIVILEGES IN SCHEMA auth GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO dev;

CREATE TABLE IF NOT EXISTS auth.accounts (
    id UUID PRIMARY KEY,

    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    verified_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_accounts_email 
    ON auth.accounts(email) 
    STORING (password_hash, created_at, updated_at, deleted_at);

CREATE TABLE IF NOT EXISTS auth.organizations (
    id UUID PRIMARY KEY,

    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_organizations_name
    ON auth.organizations(name);

CREATE TABLE IF NOT EXISTS auth.mfa_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES auth.accounts(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,
    
    method_type TEXT NOT NULL, 
    secret TEXT NOT NULL, 
    
    created_at TIMESTAMPTZ DEFAULT now(),
    last_used_at TIMESTAMPTZ,
    
    UNIQUE(account_id, organization_id, method_type)
);

ALTER TABLE auth.mfa_methods ENABLE ROW LEVEL SECURITY;
CREATE POLICY auth_mfa_isolation_policy ON auth.mfa_methods
    USING (organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID);

CREATE TABLE IF NOT EXISTS auth.members (
    account_id UUID NOT NULL REFERENCES auth.accounts(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,

    given_name TEXT NOT NULL,
    family_name TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    gender INT4 NOT NULL DEFAULT 0,
    birthdate DATE NOT NULL,
    picture TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    verified_at TIMESTAMPTZ,

    PRIMARY KEY (account_id, organization_id)
);

CREATE INDEX IF NOT EXISTS idx_members_organization_id
    ON auth.members(organization_id)
    STORING (created_at, updated_at);

ALTER TABLE auth.members ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_members_isolation_policy
    ON auth.members
    AS PERMISSIVE
    FOR ALL
USING (
        (NULLIF(current_setting('auth.organization_id', true), '')::UUID IS NOT NULL 
         OR NULLIF(current_setting('auth.account_id', true), '')::UUID IS NOT NULL)
        
        AND
        
        (organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID 
         OR NULLIF(current_setting('auth.organization_id', true), '')::UUID IS NULL)
        
        AND
        
        (account_id = NULLIF(current_setting('auth.account_id', true), '')::UUID 
         OR NULLIF(current_setting('auth.account_id', true), '')::UUID IS NULL)
    )
    WITH CHECK (
        (NULLIF(current_setting('auth.organization_id', true), '')::UUID IS NOT NULL 
         OR NULLIF(current_setting('auth.account_id', true), '')::UUID IS NOT NULL)
        
        AND

        (organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID 
         OR NULLIF(current_setting('auth.organization_id', true), '')::UUID IS NULL)
        
        AND

        (account_id = NULLIF(current_setting('auth.account_id', true), '')::UUID 
         OR NULLIF(current_setting('auth.account_id', true), '')::UUID IS NULL)
    );

CREATE TABLE IF NOT EXISTS auth.groups (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,

    name TEXT NOT NULL,
    description TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    UNIQUE (id, organization_id),
    UNIQUE (organization_id, name)
);

CREATE INDEX IF NOT EXISTS idx_groups_organization_id
    ON auth.groups(organization_id)
    STORING (name, description, created_at, updated_at);

ALTER TABLE auth.groups ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_groups_isolation_policy
    ON auth.groups
    AS PERMISSIVE
    FOR ALL
    USING (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    )
    WITH CHECK (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    );

CREATE TABLE IF NOT EXISTS auth.members_groups (
    group_id UUID NOT NULL,
    account_id UUID NOT NULL,
    organization_id UUID NOT NULL,
    
    CONSTRAINT fk_group_id FOREIGN KEY (group_id, organization_id)
        REFERENCES auth.groups(id, organization_id) ON DELETE CASCADE,
    CONSTRAINT fk_members FOREIGN KEY (account_id, organization_id) 
        REFERENCES auth.members(account_id, organization_id) ON DELETE CASCADE,

    PRIMARY KEY (group_id, account_id, organization_id)
);

CREATE INDEX IF NOT EXISTS idx_members_groups_account_id 
    ON auth.members_groups(account_id);
CREATE INDEX IF NOT EXISTS idx_members_groups_organization_id 
    ON auth.members_groups(organization_id);

ALTER TABLE auth.members_groups ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_members_groups_isolation_policy
    ON auth.members_groups
    AS PERMISSIVE
    FOR ALL
    USING (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
        AND 
        account_id = NULLIF(current_setting('auth.account_id', true), '')::UUID
    )
    WITH CHECK (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
        AND 
        account_id = NULLIF(current_setting('auth.account_id', true), '')::UUID
    );

CREATE TABLE IF NOT EXISTS auth.roles (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,

    name TEXT NOT NULL,
    description TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    UNIQUE (id, organization_id),
    UNIQUE (organization_id, name)
);

CREATE INDEX IF NOT EXISTS idx_roles_organization_id
    ON auth.roles(organization_id)
    STORING (name, description, created_at, updated_at);

ALTER TABLE auth.roles ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_roles_isolation_policy
    ON auth.roles
    AS PERMISSIVE
    FOR ALL
    USING (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    )
    WITH CHECK (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    );

CREATE TABLE IF NOT EXISTS auth.members_roles (
    role_id UUID NOT NULL,
    account_id UUID NOT NULL,
    organization_id UUID NOT NULL,
    
    CONSTRAINT fk_roles FOREIGN KEY (role_id, organization_id)
        REFERENCES auth.roles(id, organization_id) ON DELETE CASCADE,
    CONSTRAINT fk_members FOREIGN KEY (account_id, organization_id) 
        REFERENCES auth.members(account_id, organization_id) ON DELETE CASCADE,

    PRIMARY KEY (role_id, account_id, organization_id)
);

CREATE INDEX IF NOT EXISTS idx_members_roles_account_id 
    ON auth.members_roles(account_id);
CREATE INDEX IF NOT EXISTS idx_members_roles_organization_id 
    ON auth.members_roles(organization_id);

ALTER TABLE auth.members_roles ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_members_roles_isolation_policy
    ON auth.members_roles
    AS PERMISSIVE
    FOR ALL
    USING (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
        AND
        account_id = NULLIF(current_setting('auth.account_id', true), '')::UUID
    )
    WITH CHECK (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
        AND 
        account_id = NULLIF(current_setting('auth.account_id', true), '')::UUID
    );

CREATE TABLE IF NOT EXISTS auth.permissions (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,

    scope TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    UNIQUE (id, organization_id),
    UNIQUE (organization_id, scope)
);

CREATE INDEX IF NOT EXISTS idx_permissions_organization_id
    ON auth.permissions(organization_id)
    STORING (scope, name, description, created_at, updated_at);

ALTER TABLE auth.permissions ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_permissions_isolation_policy
    ON auth.permissions
    AS PERMISSIVE
    FOR ALL
    USING (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    )
    WITH CHECK (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    );

CREATE TABLE IF NOT EXISTS auth.roles_permissions (
    permission_id UUID NOT NULL,
    role_id UUID NOT NULL,
    organization_id UUID NOT NULL,
    
    CONSTRAINT fk_permissions FOREIGN KEY (permission_id, organization_id)
        REFERENCES auth.permissions(id, organization_id) ON DELETE CASCADE,
    CONSTRAINT fk_roles FOREIGN KEY (role_id, organization_id) 
        REFERENCES auth.roles(id, organization_id) ON DELETE CASCADE,

    PRIMARY KEY (permission_id, role_id, organization_id)
);

CREATE INDEX IF NOT EXISTS idx_roles_permissions_role_id
    ON auth.roles_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_roles_permissions_organization_id 
    ON auth.roles_permissions(organization_id);

ALTER TABLE auth.roles_permissions ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY;
CREATE POLICY auth_roles_permissions_isolation_policy
    ON auth.roles_permissions
    AS PERMISSIVE
    FOR ALL
    USING (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    )
    WITH CHECK (
        organization_id = NULLIF(current_setting('auth.organization_id', true), '')::UUID
    );
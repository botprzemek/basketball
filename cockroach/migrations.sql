DROP DATABASE IF EXISTS dev CASCADE;

CREATE DATABASE IF NOT EXISTS dev;

USE dev;

CREATE SCHEMA IF NOT EXISTS auth;

CREATE TABLE IF NOT EXISTS auth.accounts (
    id UUID PRIMARY KEY,
    email STRING NOT NULL UNIQUE,
    password_hash STRING NOT NULL,
    first_name STRING NOT NULL,
    last_name STRING NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_accounts_email 
    ON auth.accounts(email) 
    STORING (password_hash, first_name, last_name, is_active, created_at, updated_at);

CREATE TABLE IF NOT EXISTS auth.organizations (
    id UUID PRIMARY KEY,
    name STRING NOT NULL,
    slug STRING NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_organizations_name
    ON auth.organizations(name);

CREATE TABLE IF NOT EXISTS auth.identities (
    account_id UUID NOT NULL REFERENCES auth.accounts(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,

    PRIMARY KEY (account_id, organization_id)
);

CREATE INDEX IF NOT EXISTS idx_identities_organization_id
    ON auth.identities(organization_id)
    STORING (created_at, updated_at);

ALTER TABLE auth.identities ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy
    ON auth.identities
    AS RESTRICTIVE
    USING (organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID);

CREATE TABLE IF NOT EXISTS auth.roles (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,
    name STRING NOT NULL,
    description STRING NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,

    UNIQUE (id, organization_id),
    UNIQUE (organization_id, name)
);

CREATE INDEX IF NOT EXISTS idx_roles_organization_id
    ON auth.roles(organization_id)
    STORING (name, description, created_at, updated_at);

ALTER TABLE auth.roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy
    ON auth.roles
    AS RESTRICTIVE
    USING (organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID);

CREATE TABLE IF NOT EXISTS auth.identities_roles (
    role_id UUID NOT NULL,
    account_id UUID NOT NULL,
    organization_id UUID NOT NULL,
    
    CONSTRAINT fk_roles FOREIGN KEY (role_id, organization_id)
        REFERENCES auth.roles(id, organization_id) ON DELETE CASCADE,
    CONSTRAINT fk_identities FOREIGN KEY (account_id, organization_id) 
        REFERENCES auth.identities(account_id, organization_id) ON DELETE CASCADE,

    PRIMARY KEY (role_id, account_id, organization_id)
);

CREATE INDEX IF NOT EXISTS idx_identities_roles_account_id 
    ON auth.identities_roles(account_id);
CREATE INDEX IF NOT EXISTS idx_identities_roles_organization_id 
    ON auth.identities_roles(organization_id);

ALTER TABLE auth.identities_roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy
    ON auth.identities_roles
    AS RESTRICTIVE
    USING (organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID);

    USE dev;

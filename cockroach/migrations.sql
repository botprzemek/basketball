DROP DATABASE IF EXISTS dev CASCADE;

CREATE DATABASE IF NOT EXISTS dev;

USE dev;

CREATE SCHEMA IF NOT EXISTS auth;

CREATE TABLE IF NOT EXISTS auth.accounts (
    id UUID PRIMARY KEY,
    email STRING UNIQUE NOT NULL,
    password_hash STRING NOT NULL,
    first_name STRING NOT NULL,
    last_name STRING NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_accounts_email ON auth.accounts(email);

CREATE TABLE IF NOT EXISTS auth.organizations (
    id UUID PRIMARY KEY,
    name STRING NOT NULL,
    slug STRING UNIQUE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_organizations_name ON auth.organizations(name);
CREATE INDEX IF NOT EXISTS idx_organizations_slug ON auth.organizations(slug DESC);

CREATE TABLE IF NOT EXISTS auth.identities (
    account_id UUID NOT NULL REFERENCES auth.accounts(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    PRIMARY KEY (account_id, organization_id)
);

CREATE TABLE IF NOT EXISTS auth.roles (
    id UUID PRIMARY KEY,
    name STRING NOT NULL UNIQUE,
    description STRING NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS auth.identities_roles (
    role_id UUID NOT NULL REFERENCES auth.roles(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES auth.accounts(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES auth.organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, account_id, organization_id)
);
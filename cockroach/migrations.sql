DROP DATABASE IF EXISTS dev CASCADE;

CREATE DATABASE IF NOT EXISTS dev;

USE dev;

CREATE SCHEMA IF NOT EXISTS basketball;

CREATE TABLE IF NOT EXISTS basketball.accounts (
    id UUID PRIMARY KEY,
    email STRING UNIQUE NOT NULL,
    password_hash STRING NOT NULL,
    first_name STRING NOT NULL,
    last_name STRING NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_accounts_email ON basketball.accounts(email);
CREATE INDEX IF NOT EXISTS idx_accounts_created_at ON basketball.accounts(created_at DESC);

CREATE TABLE IF NOT EXISTS basketball.organizations (
    id UUID PRIMARY KEY,
    name STRING NOT NULL,
    slug STRING UNIQUE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_organizations_name ON basketball.organizations(name);
CREATE INDEX IF NOT EXISTS idx_organizations_slug ON basketball.organizations(slug DESC);
CREATE INDEX IF NOT EXISTS idx_organizations_created_at ON basketball.organizations(created_at DESC);

CREATE TABLE IF NOT EXISTS basketball.identities (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES basketball.organizations(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES basketball.accounts(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    UNIQUE(organization_id, account_id)
);

CREATE INDEX IF NOT EXISTS idx_identities_organization_id ON basketball.identities(organization_id);
CREATE INDEX IF NOT EXISTS idx_identities_account_id ON basketball.identities(account_id);
CREATE INDEX IF NOT EXISTS idx_identities_created_at ON basketball.identities(created_at DESC);
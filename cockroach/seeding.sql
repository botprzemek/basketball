USE dev;

TRUNCATE auth.roles CASCADE;
TRUNCATE auth.identities CASCADE;
TRUNCATE auth.organizations CASCADE;
TRUNCATE auth.accounts CASCADE;

INSERT INTO auth.accounts (id, email, password_hash, first_name, last_name)
VALUES
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'v@nightcity.net',
        '$argon2id$v=19$m=19456,t=2,p=1$eo4MYfvcPV93Xh3r8ErGsQ$C7dFh9TPVNU0XZkusoF1/uyO0vvZULabJuDJinJu4EI', 
        'Vincent',
        'Wilson'
    )
ON CONFLICT (email) DO NOTHING;

INSERT INTO auth.organizations (id, name, slug)
VALUES
    (
        'b2bc3456-1234-5678-90ab-cdef12345678',
        'Arasaka Corporation',
        'arasaka'
    ),
    (
        'c3cd4567-2345-6789-01bc-defa23456789',
        'Militech International Armaments',
        'militech'
    )
ON CONFLICT (slug) DO NOTHING;

INSERT INTO auth.identities (account_id, organization_id)
VALUES
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'b2bc3456-1234-5678-90ab-cdef12345678'
    ),
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'c3cd4567-2345-6789-01bc-defa23456789'
    )
ON CONFLICT (account_id, organization_id) DO NOTHING;

INSERT INTO auth.roles (id, organization_id, name, description)
VALUES
    (
        '019ed397-bb71-7000-8804-d02f5b45f448',
        'b2bc3456-1234-5678-90ab-cdef12345678',
        'solo',
        'Standard Enforcer'
    ),
    (
        '019ed397-bb72-7476-880e-0d12fe84411d',
        'c3cd4567-2345-6789-01bc-defa23456789',
        'netrunner',
        'Cybernetic Specialist'
    )
ON CONFLICT (organization_id, name) DO NOTHING;

INSERT INTO auth.identities_roles (role_id, account_id, organization_id)
VALUES
    (
        '019ed397-bb71-7000-8804-d02f5b45f448',
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'b2bc3456-1234-5678-90ab-cdef12345678'
    ),
    (
        '019ed397-bb72-7476-880e-0d12fe84411d',
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'c3cd4567-2345-6789-01bc-defa23456789'
    )
ON CONFLICT (role_id, account_id, organization_id) DO NOTHING;
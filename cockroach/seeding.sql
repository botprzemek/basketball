USE dev;

TRUNCATE auth.roles_permissions CASCADE;
TRUNCATE auth.members_roles CASCADE;
TRUNCATE auth.members_groups CASCADE;
TRUNCATE auth.permissions CASCADE;
TRUNCATE auth.roles CASCADE;
TRUNCATE auth.groups CASCADE;
TRUNCATE auth.mfa_methods CASCADE;
TRUNCATE auth.members CASCADE;
TRUNCATE auth.organizations CASCADE;
TRUNCATE auth.accounts CASCADE;

INSERT INTO auth.accounts (id, email, password_hash, email_verified)
VALUES
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'v@nightcity.net',
        '$argon2id$v=19$m=19456,t=2,p=1$eo4MYfvcPV93Xh3r8ErGsQ$C7dFh9TPVNU0XZkusoF1/uyO0vvZULabJuDJinJu4EI',
        true
    )
ON CONFLICT (id) DO NOTHING;

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
ON CONFLICT (id) DO NOTHING;

INSERT INTO auth.members (
    account_id, organization_id, given_name, family_name, name, 
    email, email_verified, phone_number, gender, birthdate, picture
)
VALUES
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'b2bc3456-1234-5678-90ab-cdef12345678',
        'Vincent', 'Wilson', 'Vincent Wilson',
        'v.wilson@arasaka.co', true, '+12025550143', 1, '2054-12-10', 
        'https://api.dicebear.com/7.x/bottts/svg?seed=v-arasaka'
    ),
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'c3cd4567-2345-6789-01bc-defa23456789',
        'Vincent', 'Wilson', 'Vincent Wilson',
        'v.wilson@militech.com', true, '+12025550199', 1, '2054-12-10', 
        'https://api.dicebear.com/7.x/bottts/svg?seed=v-militech'
    )
ON CONFLICT (account_id, organization_id) DO NOTHING;

INSERT INTO auth.mfa_methods (account_id, organization_id, method_type, secret)
VALUES
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'b2bc3456-1234-5678-90ab-cdef12345678',
        'totp',
        'NBSWY3DPEB3W64TBNQXD2'
    ),
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'c3cd4567-2345-6789-01bc-defa23456789',
        'totp',
        'MVRXG2LSMF2GQ5LQNQXXO'
    )
ON CONFLICT (account_id, organization_id, method_type) DO NOTHING;

INSERT INTO auth.groups (id, organization_id, name, description)
VALUES
    (
        '019ed397-bb73-7000-9901-a0a0a0a0a0a1',
        'b2bc3456-1234-5678-90ab-cdef12345678',
        'Counter-Intelligence',
        'Arasaka Counter-Intel Division Team'
    ),
    (
        '019ed397-bb73-7000-9902-b0b0b0b0b0b2',
        'c3cd4567-2345-6789-01bc-defa23456789',
        'Special Ops',
        'Militech Tactical Operations Group'
    )
ON CONFLICT (id) DO NOTHING;

INSERT INTO auth.members_groups (group_id, account_id, organization_id)
VALUES
    ('019ed397-bb73-7000-9901-a0a0a0a0a0a1', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b2bc3456-1234-5678-90ab-cdef12345678'),
    ('019ed397-bb73-7000-9902-b0b0b0b0b0b2', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'c3cd4567-2345-6789-01bc-defa23456789')
ON CONFLICT (group_id, account_id, organization_id) DO NOTHING;

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
ON CONFLICT (id) DO NOTHING;

INSERT INTO auth.members_roles (role_id, account_id, organization_id)
VALUES
    ('019ed397-bb71-7000-8804-d02f5b45f448', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b2bc3456-1234-5678-90ab-cdef12345678'),
    ('019ed397-bb72-7476-880e-0d12fe84411d', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'c3cd4567-2345-6789-01bc-defa23456789')
ON CONFLICT (role_id, account_id, organization_id) DO NOTHING;

INSERT INTO auth.permissions (id, organization_id, scope, name, description)
VALUES
    (
        '019ed397-bb74-7000-a001-c0c0c0c0c0c1',
        'b2bc3456-1234-5678-90ab-cdef12345678',
        'intel',
        'read:secure',
        'Access to corporate data feeds'
    ),
    (
        '019ed397-bb74-7000-a002-d0d0d0d0d0d2',
        'c3cd4567-2345-6789-01bc-defa23456789',
        'subnets',
        'breach',
        'Permission to execute override protocols'
    )
ON CONFLICT (id) DO NOTHING;

INSERT INTO auth.roles_permissions (permission_id, role_id, organization_id)
VALUES
    ('019ed397-bb74-7000-a001-c0c0c0c0c0c1', '019ed397-bb71-7000-8804-d02f5b45f448', 'b2bc3456-1234-5678-90ab-cdef12345678'),
    ('019ed397-bb74-7000-a002-d0d0d0d0d0d2', '019ed397-bb72-7476-880e-0d12fe84411d', 'c3cd4567-2345-6789-01bc-defa23456789')
ON CONFLICT (permission_id, role_id, organization_id) DO NOTHING;
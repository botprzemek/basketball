interface LoginEvent {
    email: string;
    password: string;
}

interface ContextOrganization {
    organization: Organization;
    member: Member;
}

interface ContextSelectEvent {
    organization: Organization;
}

interface ContextCurrent {
    organizationId: string;
    accountId: string;
}

interface Organization {
    id: string;

    name: string;
    slug: string;

    createdAt: string;
    updatedAt: string | null;
    deletedAt: string | null;
}

interface Member {
    accountId: string;
    organizationId: string;

    givenName: string;
    familyName: string;
    name: string;
    email: string;
    phoneNumber: string;
    gender: number;
    birthdate: string;
    picture: string;

    createdAt: string;
    updatedAt: string | null;
    verifiedAt: string | null;
}

interface Group {
    id: string;
    organizationId: string;

    name: string;
    description: string;

    createdAt: string;
    updatedAt: string | null;
}

interface Role {
    id: string;
    organizationId: string;

    name: string;
    description: string;

    createdAt: string;
    updatedAt: string | null;
}

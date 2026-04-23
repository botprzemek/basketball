interface RegisterCredentials {
  email: string;
  password: string;
  firstName: string;
  lastName: string;
}

interface LoginCredentials {
  email: string;
  password: string;
}

interface AccountIdentity {
  identityId: string;
  organizationId: string;
  organizationName: string;
  organizationSlug: string;
  createdAt: Date;
  updatedAt?: Date;
}

interface Organization {
  id: string;
  name: string;
  slug: string;
}

interface Member {
  id: string;
  email: string;
  firstName: string;
  lastName: string;
  joinedAt: Date;
}

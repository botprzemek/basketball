export const useIdentity = async () => {
    const { context } = useAuth();
    const { data: identity } = await useAPI(
        `/organizations/${context.value.organizationId}/identity`,
        {
            key: "context_organization_identity",
        },
    );

    return identity;
};

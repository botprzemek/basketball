export const useOrganization = async () => {
    const { context } = useAuth();
    const { data: organization } = await useAPI(
        `/organizations/${context.value.organizationId}`,
        {
            key: "context_organization",
        },
    );
    const { data: roles } = await useAPI(
        `/organizations/${context.value.organizationId}/roles`,
        {
            key: "context_organization_roles",
        },
    );

    return {
        organization,
        roles,
    };
};

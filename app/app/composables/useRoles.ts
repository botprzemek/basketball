export const useRoles = async () => {
    const { context } = useAuth();
    const { data: roles } = await useAPI(
        `/organizations/${context.value.organizationId}/roles`,
        {
            key: "context_organization_roles",
        },
    );

    return roles;
};
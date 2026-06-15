export const useRoles = async () => {
    const { context } = useAuth();
    const { data: roles } = await useAPI(
        `/api/v1/organizations/${context.value.organizationId}/roles`,
        {
            key: "organization_roles",
        },
    );

    return roles;
};

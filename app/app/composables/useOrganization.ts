export const useOrganization = async () => {
    const { context } = useAuth();
    const { data: organization } = await useAPI(
        `/organizations/${context.value.organizationId}`,
        {
            key: "context_organization",
        },
    );

    return organization;
};

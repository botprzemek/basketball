export const useOrganization = async () => {
    const { context } = useAuth();
    const { data: organization } = await useAPI(
        `/api/v1/organizations/${context.value.organizationId}`,
        {
            key: "organization",
        },
    );

    return organization;
};

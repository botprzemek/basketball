export const useMembers = async () => {
    const { context } = useAuth();
    const { data: members } = await useAPI(
        `/api/v1/organizations/${context.value.organizationId}/members`,
        {
            key: "organization_members",
        },
    );

    return members;
};

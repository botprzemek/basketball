export const useMembers = async () => {
    const { context } = useAuth();
    const { data: members } = await useAPI(
        `/organizations/${context.value.organizationId}/members`,
        {
            key: "context_members",
        },
    );

    return members;
};

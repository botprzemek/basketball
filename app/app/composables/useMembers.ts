export const useMembers = async () => {
    const { data: members } = await useContext("/members", {
        key: "organization-members",
    });

    return members;
};

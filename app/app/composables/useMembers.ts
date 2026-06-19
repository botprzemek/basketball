export const useMembers = async () => {
    const { data: members } = await useContext<Array<Member>>("/members", {
        key: "organization-members",
    });

    return members;
};

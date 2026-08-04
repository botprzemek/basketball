export const useGroups = async () => {
    const { data: groups } = await useContext<Array<Group>>("/group", {
        key: "organization-groups",
    });

    return groups;
};

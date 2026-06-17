export const useRoles = async () => {
    const { data: roles } = await useContext("/roles", {
        key: "organization-roles",
    });

    return roles;
};

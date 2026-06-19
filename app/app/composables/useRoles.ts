export const useRoles = async () => {
    const { data: roles } = await useContext<Array<Role>>("/roles", {
        key: "organization-roles",
    });

    return roles;
};

export const useOrganization = async () => {
    const { data: organization } = await useContext<Organization>("", {
        key: "organization",
    });

    return organization;
};

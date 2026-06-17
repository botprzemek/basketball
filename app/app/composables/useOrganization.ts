export const useOrganization = async () => {
    const { data: organization } = await useContext("", {
        key: "organization",
    });

    return organization;
};

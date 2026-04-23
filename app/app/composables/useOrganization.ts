export const useOrganization = () => {
  const getDetails = async () => {
    const { data: organization } = await useFetch("/api/organization", {
      key: "organization",
    });

    return organization;
  };

  const getMembers = async () => {
    const { data: members } = await useFetch("/api/organization/members", {
      key: "organization-members",
    });

    return members;
  };

  return {
    getDetails,
    getMembers,
  };
};

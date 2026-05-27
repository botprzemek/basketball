import { defineEventHandler, createError, getCookie } from "h3";

export default defineEventHandler((event) => {
    const token = getCookie(event, "token");

    if (!token) {
        throw createError({
            statusCode: 401,
            statusMessage: "Unauthorized - No session found",
        });
    }

    if (token !== "v-is-the-best") {
        throw createError({
            statusCode: 401,
            statusMessage: "Session expired or invalid token",
        });
    }

    return {
        id: "user-001",
        email: "v@nightcity.net",
        name: "V",
        organizations: [
            {
                id: "org-001",
                name: "Arasaka Corporation",
                slug: "arasaka-corp",
                role: "Owner",
                permissions: ["org:create"],
            },
        ],
    };
});

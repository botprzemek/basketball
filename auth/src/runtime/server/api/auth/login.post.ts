import { defineEventHandler, readBody, createError, setCookie } from "h3";

export default defineEventHandler(async (event) => {
    const body = await readBody(event);

    if (body.email !== "v@nightcity.net") {
        throw createError({
            statusCode: 401,
            statusMessage: "Invalid credentials",
        });
    }

    setCookie(event, "token", "v-is-the-best", { httpOnly: true });

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

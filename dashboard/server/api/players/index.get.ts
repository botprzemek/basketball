export default defineEventHandler(async (event) => {
    const cookies = getRequestHeaders(event)["cookie"];

    if (cookies) {
        return;
    }

    const data = await fetch("http://localhost:3000/v1/players", {
        method: "GET",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
            cookie: `${cookies}`,
        },
    });
});

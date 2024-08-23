export default defineEventHandler(async (event) => {
    const { cookie } = getRequestHeaders(event);

    return fetch("http://localhost:3000/v1/players", {
        method: "GET",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
            cookie: `${cookie}`
        }
    });
});

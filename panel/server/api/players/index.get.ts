export default defineEventHandler(async (event) => {
	return fetch("http://127.0.0.1:3000/v1/players", {
		method: "GET",
		mode: "cors",
		credentials: "include",
		headers: {
			"Content-Type": "application/json",
		},
	});
});
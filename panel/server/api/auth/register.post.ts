export default defineEventHandler(async (event) => {
	const {email, password} = await readBody(event);

	if (!email || !password) {
		return;
	}

	return fetch("http://127.0.0.1:3000/v1/auth/register", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({
			email,
			password
		})
	});
});
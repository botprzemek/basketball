export default defineEventHandler(async (event) => {
	return await (await fetch("http://basketball-api:3000/")).json() ?? [];
});
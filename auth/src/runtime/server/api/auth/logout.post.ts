import { defineEventHandler, setCookie } from "h3";

export default defineEventHandler(async (event) => {
    setCookie(event, "token", "", { maxAge: 0 });
});

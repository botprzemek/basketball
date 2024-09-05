import "#internal/nitro/virtual/polyfill";
import { normalize } from "pathe";
import { nitroApp } from "../app.mjs";
async function cli() {
  const url = process.argv[2] || "/";
  const debug = (label, ...args) => console.debug(`> ${label}:`, ...args);
  const r = await nitroApp.localCall({ url });
  debug("URL", url);
  debug("StatusCode", r.status);
  debug("StatusMessage", r.statusText);
  for (const header of Object.entries(r.headers)) {
    debug(header[0], header[1]);
  }
  console.log("\n", r.body.toString());
}
if (process.argv.some((arg) => import.meta.url.includes(normalize(arg)))) {
  cli().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}

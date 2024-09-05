import { joinURL } from "ufo";
import { defineNuxtPlugin } from "#app/nuxt";
import { prerenderRoutes } from "#app/composables/ssr";
import _routes from "#build/routes";
import routerOptions from "#build/router.options";
let routes;
export default defineNuxtPlugin(async () => {
  if (!import.meta.server || !import.meta.prerender || routerOptions.hashMode) {
    return;
  }
  if (routes && !routes.length) {
    return;
  }
  routes ||= Array.from(processRoutes(await routerOptions.routes?.(_routes) ?? _routes));
  const batch = routes.splice(0, 10);
  prerenderRoutes(batch);
});
const OPTIONAL_PARAM_RE = /^\/?:.*(?:\?|\(\.\*\)\*)$/;
function processRoutes(routes2, currentPath = "/", routesToPrerender = /* @__PURE__ */ new Set()) {
  for (const route of routes2) {
    if (OPTIONAL_PARAM_RE.test(route.path) && !route.children?.length) {
      routesToPrerender.add(currentPath);
    }
    if (route.path.includes(":")) {
      continue;
    }
    const fullPath = joinURL(currentPath, route.path);
    routesToPrerender.add(fullPath);
    if (route.children) {
      processRoutes(route.children, fullPath, routesToPrerender);
    }
  }
  return routesToPrerender;
}

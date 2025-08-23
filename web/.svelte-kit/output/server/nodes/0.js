

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.9EqizrXa.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/DC0M3G7w.js","_app/immutable/chunks/C9FsBCx3.js","_app/immutable/chunks/Bc3bfzMI.js","_app/immutable/chunks/BAvnsGhT.js","_app/immutable/chunks/CYHqH_Yx.js","_app/immutable/chunks/CXHqpH_5.js"];
export const stylesheets = ["_app/immutable/assets/0.DGYA-gKs.css"];
export const fonts = [];

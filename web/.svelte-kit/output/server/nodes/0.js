

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.DuOHhtoJ.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/CsDgUE8k.js","_app/immutable/chunks/BhbyoVt2.js","_app/immutable/chunks/BaqkDNVs.js","_app/immutable/chunks/O1SpF5sJ.js","_app/immutable/chunks/2D9dYVSX.js"];
export const stylesheets = ["_app/immutable/assets/0.bq93w-09.css"];
export const fonts = [];

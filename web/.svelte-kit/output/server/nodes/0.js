

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.BHidSqbH.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/6OI8ooP4.js","_app/immutable/chunks/BxV-5GWJ.js","_app/immutable/chunks/DzFvVnrv.js","_app/immutable/chunks/e_jWNfsI.js","_app/immutable/chunks/DuCBgHP2.js","_app/immutable/chunks/oA4OtbPZ.js"];
export const stylesheets = ["_app/immutable/assets/0.bq93w-09.css"];
export const fonts = [];

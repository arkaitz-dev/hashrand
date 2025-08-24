

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.CWR43xbY.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/CTZJp9_Z.js","_app/immutable/chunks/BZkW1K7g.js","_app/immutable/chunks/DJTM-w6B.js","_app/immutable/chunks/C0gCOpH7.js","_app/immutable/chunks/Cknr86q3.js","_app/immutable/chunks/M35b4Yb-.js"];
export const stylesheets = ["_app/immutable/assets/0.sjTCT2-E.css"];
export const fonts = [];



export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.eu9h4NjV.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/BUZ_d3pq.js","_app/immutable/chunks/9Nf0Me7R.js","_app/immutable/chunks/CxZvJfgd.js","_app/immutable/chunks/D_S3Tegi.js","_app/immutable/chunks/zGbMtZmf.js"];
export const stylesheets = ["_app/immutable/assets/0.hUIAS9MX.css"];
export const fonts = [];

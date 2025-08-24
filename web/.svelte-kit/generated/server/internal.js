
import root from '../root.js';
import { set_building, set_prerendering } from '__sveltekit/environment';
import { set_assets } from '__sveltekit/paths';
import { set_manifest, set_read_implementation } from '__sveltekit/server';
import { set_private_env, set_public_env, set_safe_public_env } from '../../../node_modules/@sveltejs/kit/src/runtime/shared-server.js';

export const options = {
	app_template_contains_nonce: false,
	csp: {"mode":"auto","directives":{"upgrade-insecure-requests":false,"block-all-mixed-content":false},"reportOnly":{"upgrade-insecure-requests":false,"block-all-mixed-content":false}},
	csrf_check_origin: true,
	embedded: false,
	env_public_prefix: 'PUBLIC_',
	env_private_prefix: '',
	hash_routing: false,
	hooks: null, // added lazily, via `get_hooks`
	preload_strategy: "modulepreload",
	root,
	service_worker: false,
	service_worker_options: undefined,
	templates: {
		app: ({ head, body, assets, nonce, env }) => "<!doctype html>\n<html lang=\"en\" class=\"%sveltekit.theme%\">\n\t<head>\n\t\t<meta charset=\"utf-8\" />\n\t\t<link rel=\"icon\" href=\"" + assets + "/favicon.png\" />\n\t\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\t\t<meta\n\t\t\tname=\"description\"\n\t\t\tcontent=\"Professional hash, password, and API key generator with customizable parameters\"\n\t\t/>\n\t\t<meta\n\t\t\tname=\"keywords\"\n\t\t\tcontent=\"hash generator, password generator, API key generator, random, security, cryptography\"\n\t\t/>\n\t\t<meta name=\"author\" content=\"HashRand Spin\" />\n\n\t\t<!-- Open Graph / Facebook -->\n\t\t<meta property=\"og:type\" content=\"website\" />\n\t\t<meta property=\"og:title\" content=\"Hash Generator - Professional Random Generation Tool\" />\n\t\t<meta\n\t\t\tproperty=\"og:description\"\n\t\t\tcontent=\"Professional hash, password, and API key generator with customizable parameters\"\n\t\t/>\n\n\t\t<!-- Twitter -->\n\t\t<meta name=\"twitter:card\" content=\"summary\" />\n\t\t<meta name=\"twitter:title\" content=\"Hash Generator - Professional Random Generation Tool\" />\n\t\t<meta\n\t\t\tname=\"twitter:description\"\n\t\t\tcontent=\"Professional hash, password, and API key generator with customizable parameters\"\n\t\t/>\n\n\t\t<!-- Deferred sprite loader -->\n\t\t<script defer>\n\t\t\t// Global sprite loading state\n\t\t\twindow.__SPRITE_STATE__ = {\n\t\t\t\tloaded: false,\n\t\t\t\tloading: true,\n\t\t\t\terror: false\n\t\t\t};\n\n\t\t\t// Load sprite immediately\n\t\t\tfunction loadSprite() {\n\t\t\t\tfetch('" + assets + "/icons-sprite.svg')\n\t\t\t\t\t.then((response) => {\n\t\t\t\t\t\tif (!response.ok) {\n\t\t\t\t\t\t\tthrow new Error(`HTTP ${response.status}`);\n\t\t\t\t\t\t}\n\t\t\t\t\t\treturn response.text();\n\t\t\t\t\t})\n\t\t\t\t\t.then((svgContent) => {\n\t\t\t\t\t\t// Inject sprite SVG directly at end of body\n\t\t\t\t\t\tconst parser = new DOMParser();\n\t\t\t\t\t\tconst svgDoc = parser.parseFromString(svgContent, 'image/svg+xml');\n\t\t\t\t\t\tconst svgElement = svgDoc.documentElement;\n\t\t\t\t\t\tsvgElement.style.display = 'none';\n\t\t\t\t\t\tsvgElement.id = 'icons-sprite-cache';\n\t\t\t\t\t\tdocument.body.appendChild(svgElement);\n\n\t\t\t\t\t\t// Update global state\n\t\t\t\t\t\twindow.__SPRITE_STATE__ = {\n\t\t\t\t\t\t\tloaded: true,\n\t\t\t\t\t\t\tloading: false,\n\t\t\t\t\t\t\terror: false\n\t\t\t\t\t\t};\n\n\t\t\t\t\t\t// Dispatch custom event for Svelte reactivity\n\t\t\t\t\t\twindow.dispatchEvent(new CustomEvent('sprite-loaded'));\n\t\t\t\t\t})\n\t\t\t\t\t.catch((error) => {\n\t\t\t\t\t\tconsole.error('[SpriteLoader] Failed to load sprite:', error);\n\t\t\t\t\t\twindow.__SPRITE_STATE__ = {\n\t\t\t\t\t\t\tloaded: false,\n\t\t\t\t\t\t\tloading: false,\n\t\t\t\t\t\t\terror: true\n\t\t\t\t\t\t};\n\n\t\t\t\t\t\t// Dispatch error event\n\t\t\t\t\t\twindow.dispatchEvent(new CustomEvent('sprite-error', { detail: error }));\n\t\t\t\t\t});\n\t\t\t}\n\n\t\t\t// Start loading\n\t\t\tloadSprite();\n\t\t</script>\n\n\t\t<!-- Preload critical fonts -->\n\t\t<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\" />\n\t\t<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin />\n\t\t<link\n\t\t\thref=\"https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap\"\n\t\t\trel=\"stylesheet\"\n\t\t/>\n\n\t\t" + head + "\n\t</head>\n\t<body\n\t\tdata-sveltekit-preload-data=\"hover\"\n\t\tclass=\"font-sans antialiased bg-white dark:bg-gray-900 text-gray-900 dark:text-white\"\n\t>\n\t\t<div style=\"display: contents\">" + body + "</div>\n\t</body>\n</html>\n",
		error: ({ status, message }) => "<!doctype html>\n<html lang=\"en\">\n\t<head>\n\t\t<meta charset=\"utf-8\" />\n\t\t<title>" + message + "</title>\n\n\t\t<style>\n\t\t\tbody {\n\t\t\t\t--bg: white;\n\t\t\t\t--fg: #222;\n\t\t\t\t--divider: #ccc;\n\t\t\t\tbackground: var(--bg);\n\t\t\t\tcolor: var(--fg);\n\t\t\t\tfont-family:\n\t\t\t\t\tsystem-ui,\n\t\t\t\t\t-apple-system,\n\t\t\t\t\tBlinkMacSystemFont,\n\t\t\t\t\t'Segoe UI',\n\t\t\t\t\tRoboto,\n\t\t\t\t\tOxygen,\n\t\t\t\t\tUbuntu,\n\t\t\t\t\tCantarell,\n\t\t\t\t\t'Open Sans',\n\t\t\t\t\t'Helvetica Neue',\n\t\t\t\t\tsans-serif;\n\t\t\t\tdisplay: flex;\n\t\t\t\talign-items: center;\n\t\t\t\tjustify-content: center;\n\t\t\t\theight: 100vh;\n\t\t\t\tmargin: 0;\n\t\t\t}\n\n\t\t\t.error {\n\t\t\t\tdisplay: flex;\n\t\t\t\talign-items: center;\n\t\t\t\tmax-width: 32rem;\n\t\t\t\tmargin: 0 1rem;\n\t\t\t}\n\n\t\t\t.status {\n\t\t\t\tfont-weight: 200;\n\t\t\t\tfont-size: 3rem;\n\t\t\t\tline-height: 1;\n\t\t\t\tposition: relative;\n\t\t\t\ttop: -0.05rem;\n\t\t\t}\n\n\t\t\t.message {\n\t\t\t\tborder-left: 1px solid var(--divider);\n\t\t\t\tpadding: 0 0 0 1rem;\n\t\t\t\tmargin: 0 0 0 1rem;\n\t\t\t\tmin-height: 2.5rem;\n\t\t\t\tdisplay: flex;\n\t\t\t\talign-items: center;\n\t\t\t}\n\n\t\t\t.message h1 {\n\t\t\t\tfont-weight: 400;\n\t\t\t\tfont-size: 1em;\n\t\t\t\tmargin: 0;\n\t\t\t}\n\n\t\t\t@media (prefers-color-scheme: dark) {\n\t\t\t\tbody {\n\t\t\t\t\t--bg: #222;\n\t\t\t\t\t--fg: #ddd;\n\t\t\t\t\t--divider: #666;\n\t\t\t\t}\n\t\t\t}\n\t\t</style>\n\t</head>\n\t<body>\n\t\t<div class=\"error\">\n\t\t\t<span class=\"status\">" + status + "</span>\n\t\t\t<div class=\"message\">\n\t\t\t\t<h1>" + message + "</h1>\n\t\t\t</div>\n\t\t</div>\n\t</body>\n</html>\n"
	},
	version_hash: "vwmigs"
};

export async function get_hooks() {
	let handle;
	let handleFetch;
	let handleError;
	let handleValidationError;
	let init;
	

	let reroute;
	let transport;
	

	return {
		handle,
		handleFetch,
		handleError,
		handleValidationError,
		init,
		reroute,
		transport
	};
}

export { set_assets, set_building, set_manifest, set_prerendering, set_private_env, set_public_env, set_read_implementation, set_safe_public_env };

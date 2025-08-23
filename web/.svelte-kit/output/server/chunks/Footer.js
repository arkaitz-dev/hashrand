import { N as escape_html, E as store_get, J as unsubscribe_stores, D as pop, A as push } from "./index.js";
import { I as Icon, _ } from "./rtl.js";
function Footer($$payload, $$props) {
  push();
  var $$store_subs;
  $$payload.out.push(`<div class="text-center mt-8"><div class="text-sm text-gray-500 dark:text-gray-400">`);
  {
    $$payload.out.push("<!--[!-->");
    {
      $$payload.out.push("<!--[!-->");
      $$payload.out.push(`<span>${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.versionsUnavailable"))}</span>`);
    }
    $$payload.out.push(`<!--]-->`);
  }
  $$payload.out.push(`<!--]--></div> <div class="text-xs text-gray-400 dark:text-gray-500 mt-2 flex items-center justify-center force-ltr"><span>Made with</span> `);
  Icon($$payload, { name: "heart", size: "w-3 h-3 mx-1 text-red-500" });
  $$payload.out.push(`<!----> <span>by</span> <a href="https://arkaitz.dev" target="_blank" rel="noopener noreferrer" class="ml-1 text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 hover:underline">Arkaitz Dev</a></div></div>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  Footer as F
};

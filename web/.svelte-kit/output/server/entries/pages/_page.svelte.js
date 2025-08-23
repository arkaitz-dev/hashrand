import { N as ensure_array_like, J as head, M as escape_html, D as store_get, E as attr_class, G as attr, F as stringify, I as unsubscribe_stores, B as pop, z as push } from "../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../chunks/exports.js";
import "../../chunks/utils.js";
import "../../chunks/state.svelte.js";
import "clsx";
import { _, i as isRTL } from "../../chunks/rtl.js";
import { I as Iconize, F as Footer } from "../../chunks/Footer.js";
const navigationItems = [
  {
    id: "custom",
    title: "Custom Hash",
    description: "Generate customized random hashes with various parameters",
    path: "/custom",
    icon: "🎲"
  },
  {
    id: "password",
    title: "Secure Password",
    description: "Generate secure passwords with guaranteed entropy",
    path: "/password",
    icon: "🔐"
  },
  {
    id: "api-key",
    title: "API Key",
    description: "Generate API keys with ak_ prefix for applications",
    path: "/api-key",
    icon: "🔑"
  }
];
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  function getTranslatedTitle(itemId) {
    switch (itemId) {
      case "custom":
        return store_get($$store_subs ??= {}, "$_", _)("custom.title");
      case "password":
        return store_get($$store_subs ??= {}, "$_", _)("password.title");
      case "api-key":
        return store_get($$store_subs ??= {}, "$_", _)("apiKey.title");
      default:
        return "";
    }
  }
  function getTranslatedDescription(itemId) {
    switch (itemId) {
      case "custom":
        return store_get($$store_subs ??= {}, "$_", _)("custom.description");
      case "password":
        return store_get($$store_subs ??= {}, "$_", _)("password.description");
      case "api-key":
        return store_get($$store_subs ??= {}, "$_", _)("apiKey.description");
      default:
        return "";
    }
  }
  const each_array = ensure_array_like(navigationItems);
  head($$payload, ($$payload2) => {
    $$payload2.title = `<title>${escape_html(store_get($$store_subs ??= {}, "$_", _)("menu.title"))} - ${escape_html(store_get($$store_subs ??= {}, "$_", _)("menu.brandName"))}</title>`;
    $$payload2.out.push(`<meta name="description"${attr("content", store_get($$store_subs ??= {}, "$_", _)("menu.description"))}/>`);
  });
  $$payload.out.push(`<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"><div class="container mx-auto px-4 py-8"><header class="text-center mb-12"><div class="inline-flex items-center justify-center w-16 h-16 bg-blue-600 rounded-full mb-6"><span class="text-2xl text-white">🎲</span></div> <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">${escape_html(store_get($$store_subs ??= {}, "$_", _)("menu.title"))}</h1> <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">${escape_html(store_get($$store_subs ??= {}, "$_", _)("menu.description"))}</p></header> <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto mb-12"><!--[-->`);
  for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
    let item = each_array[$$index];
    $$payload.out.push(`<button${attr_class(`w-full bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 cursor-pointer border border-gray-200 dark:border-gray-700 ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "text-right" : "text-left")}`)}${attr("aria-label", `${stringify(store_get($$store_subs ??= {}, "$_", _)("common.navigateTo"))} ${stringify(getTranslatedTitle(item.id))}`)}><div class="p-6"><div class="mb-4 flex items-center gap-3">`);
    Iconize($$payload, { conf: { emoji: item.icon, iconSize: "text-3xl" } });
    $$payload.out.push(`<!----> <h2 class="text-xl font-semibold text-gray-900 dark:text-white">${escape_html(getTranslatedTitle(item.id))}</h2></div> <p class="text-gray-600 dark:text-gray-300 leading-relaxed">${escape_html(getTranslatedDescription(item.id))}</p> <div${attr_class(`mt-4 inline-flex items-center text-blue-600 dark:text-blue-400 text-sm font-medium ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "rtl-float-right" : "rtl-float-left")}`)}>`);
    Iconize($$payload, {
      conf: {
        icon: "arrow-right",
        rtlIcon: "arrow-left",
        iconSize: "w-4 h-4",
        spacing: "gap-1",
        invertposition: true
      },
      children: ($$payload2) => {
        $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.choose"))}`);
      }
    });
    $$payload.out.push(`<!----></div> <div class="clear-both"></div></div></button>`);
  }
  $$payload.out.push(`<!--]--></div> `);
  Footer($$payload);
  $$payload.out.push(`<!----></div></div>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  _page as default
};

import { E as store_get, O as ensure_array_like, K as head, N as escape_html, I as attr, P as maybe_selected, F as attr_class, J as unsubscribe_stores, D as pop, A as push } from "../../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import "../../../chunks/state.svelte.js";
import { L as LoadingSpinner } from "../../../chunks/LoadingSpinner.js";
import { F as Footer } from "../../../chunks/Footer.js";
import { I as Iconize } from "../../../chunks/Iconize.js";
import { i as isLoading } from "../../../chunks/result.js";
import { _ } from "../../../chunks/rtl.js";
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  let alphabetOptions, lengthValid, prefixValid, suffixValid, formValid;
  function getDefaultParams() {
    return {
      length: 21,
      alphabet: "base58",
      prefix: "",
      suffix: "",
      raw: true
    };
  }
  let params = getDefaultParams();
  alphabetOptions = [
    {
      value: "base58",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.base58"),
      description: store_get($$store_subs ??= {}, "$_", _)("custom.bitcoinDescription")
    },
    {
      value: "no-look-alike",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.no-look-alike"),
      description: store_get($$store_subs ??= {}, "$_", _)("custom.maxReadabilityDescription")
    },
    {
      value: "full",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.full"),
      description: store_get($$store_subs ??= {}, "$_", _)("custom.completeAlphanumericDescription")
    },
    {
      value: "full-with-symbols",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.full-with-symbols"),
      description: store_get($$store_subs ??= {}, "$_", _)("custom.maxEntropyDescription")
    }
  ];
  lengthValid = params.length <= 128;
  prefixValid = true;
  suffixValid = true;
  formValid = lengthValid && prefixValid && suffixValid;
  const each_array = ensure_array_like(alphabetOptions);
  head($$payload, ($$payload2) => {
    $$payload2.title = `<title>${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.title"))}</title>`;
  });
  $$payload.out.push(`<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"><div class="container mx-auto px-4 py-8"><div class="mb-8"><div class="text-center"><div class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4"><span class="text-xl text-white">🎲</span></div> <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.title"))}</h1> <p class="text-gray-600 dark:text-gray-300">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.description"))}</p></div></div> <div class="max-w-2xl mx-auto"><div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6"><form class="space-y-6"><div><label for="length" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.length"))} (2-128)</label> <div class="flex items-center gap-4"><input type="range" id="length"${attr("value", params.length)} min="2" max="128" class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"/> <span class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${escape_html(params.length)}</span></div> `);
  if (!lengthValid) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-red-500 text-sm mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.lengthMustBeBetween"))}</p>`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--></div> <div><label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.alphabet"))}</label> <select id="alphabet" class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">`);
  $$payload.select_value = params.alphabet;
  $$payload.out.push(`<!--[-->`);
  for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
    let option = each_array[$$index];
    $$payload.out.push(`<option${attr("value", option.value)}${maybe_selected($$payload, option.value)}>${escape_html(option.label)}</option>`);
  }
  $$payload.out.push(`<!--]-->`);
  $$payload.select_value = void 0;
  $$payload.out.push(`</select> `);
  {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">${escape_html(alphabetOptions.find((o) => o.value === params.alphabet)?.description)}</p>`);
  }
  $$payload.out.push(`<!--]--></div> <div><label for="prefix" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.prefix"))} (${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.cannotExceed"))} 32 ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.characters"))})</label> <input type="text" id="prefix"${attr("value", params.prefix)} maxlength="32"${attr_class("w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white", void 0, { "border-red-500": !prefixValid })}${attr("placeholder", store_get($$store_subs ??= {}, "$_", _)("common.optionalPrefix"))}/> `);
  if (!prefixValid) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-red-500 text-sm mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.prefixCannotExceed"))}</p>`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--></div> <div><label for="suffix" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.suffix"))} (${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.cannotExceed"))} 32 ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.characters"))})</label> <input type="text" id="suffix"${attr("value", params.suffix)} maxlength="32"${attr_class("w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white", void 0, { "border-red-500": !suffixValid })}${attr("placeholder", store_get($$store_subs ??= {}, "$_", _)("common.optionalSuffix"))}/> `);
  if (!suffixValid) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-red-500 text-sm mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.suffixCannotExceed"))}</p>`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--></div> <div class="flex flex-col sm:flex-row gap-4 mt-4"><button type="submit"${attr("disabled", !formValid || store_get($$store_subs ??= {}, "$isLoading", isLoading), true)} class="flex-1 text-white bg-blue-600 hover:bg-blue-700 px-6 py-4 rounded-lg font-semibold border-none cursor-pointer hover:shadow-lg transition-all duration-200 flex items-center justify-center disabled:bg-gray-400 disabled:cursor-not-allowed disabled:shadow-none">`);
  if (store_get($$store_subs ??= {}, "$isLoading", isLoading)) {
    $$payload.out.push("<!--[-->");
    LoadingSpinner($$payload, { size: "sm", class: "mr-2" });
    $$payload.out.push(`<!----> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.loading"))}...`);
  } else {
    $$payload.out.push("<!--[!-->");
    $$payload.out.push(`${escape_html(store_get($$store_subs ??= {}, "$_", _)("custom.generateHash"))}`);
  }
  $$payload.out.push(`<!--]--></button> <button class="flex-1 bg-gray-600 hover:bg-gray-700 text-white px-6 py-4 rounded-lg font-semibold border-none cursor-pointer hover:shadow-lg transition-all duration-200 flex items-center justify-center gap-2">`);
  Iconize($$payload, {
    conf: { icon: "briefcase", iconSize: "w-5 h-5" },
    children: ($$payload2) => {
      $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.backToMenu"))}`);
    },
    $$slots: { default: true }
  });
  $$payload.out.push(`<!----></button></div></form></div></div> `);
  Footer($$payload);
  $$payload.out.push(`<!----></div></div>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  _page as default
};

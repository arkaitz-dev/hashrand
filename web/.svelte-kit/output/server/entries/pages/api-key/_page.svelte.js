import { D as store_get, N as ensure_array_like, J as head, M as escape_html, G as attr, O as maybe_selected, E as attr_class, F as stringify, I as unsubscribe_stores, B as pop, z as push } from "../../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import "../../../chunks/state.svelte.js";
import { L as LoadingSpinner } from "../../../chunks/LoadingSpinner.js";
import { I as Iconize, F as Footer } from "../../../chunks/Footer.js";
import { i as isLoading } from "../../../chunks/result.js";
import { _ } from "../../../chunks/rtl.js";
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  let alphabetOptions, minLength, lengthValid, formValid;
  function getDefaultParams() {
    return {
      length: 44,
      // Minimum for full alphabet
      alphabet: "full",
      raw: true
    };
  }
  let params = getDefaultParams();
  alphabetOptions = [
    {
      value: "full",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.full"),
      description: store_get($$store_subs ??= {}, "$_", _)("apiKey.standardAlphanumericDescription")
    },
    {
      value: "no-look-alike",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.no-look-alike"),
      description: store_get($$store_subs ??= {}, "$_", _)("apiKey.noConfusingDescription")
    }
  ];
  minLength = 44;
  lengthValid = params.length >= minLength && params.length <= 64;
  formValid = lengthValid;
  const each_array = ensure_array_like(alphabetOptions);
  head($$payload, ($$payload2) => {
    $$payload2.title = `<title>${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.title"))}</title>`;
  });
  $$payload.out.push(`<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"><div class="container mx-auto px-4 py-8"><div class="mb-8"><div class="text-center"><div class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4"><span class="text-xl text-white">🔑</span></div> <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.title"))}</h1> <p class="text-gray-600 dark:text-gray-300">${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.description"))}</p></div></div> <div class="max-w-2xl mx-auto"><div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6"><form class="space-y-6"><div><label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.alphabet"))}</label> <select id="alphabet" class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">`);
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
  $$payload.out.push(`<!--]--></div> <div><label for="length" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.length"))} (${escape_html(minLength)}-64 ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.characters"))})</label> <div class="flex items-center gap-4"><input type="range" id="length"${attr("value", params.length)}${attr("min", minLength)} max="64" class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"/> <span class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${escape_html(params.length)}</span></div> `);
  if (!lengthValid) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-red-500 text-sm mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.length"))}
								${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.mustBeBetween"))}
								${escape_html(minLength)}
								${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.and"))} 64</p>`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--> <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mt-3"><p class="text-sm text-blue-800 dark:text-blue-200"><strong>${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.format"))}:</strong> ak_ prefix + ${escape_html(params.length)}
								${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.randomCharacters"))} `);
  {
    $$payload.out.push("<!--[!-->");
    $$payload.out.push(`${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.fullAlphanumericAlphabet"))}`);
  }
  $$payload.out.push(`<!--]--> <br/><strong>${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.security"))}:</strong> `);
  {
    $$payload.out.push("<!--[!-->");
    $$payload.out.push(`${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.fullAlphanumericNote").replace("{0}", minLength.toString()))}`);
  }
  $$payload.out.push(`<!--]--></p></div></div> <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"><div class="flex items-start"><span class="text-blue-600 dark:text-blue-400 mr-2">ℹ️</span> <div class="text-sm text-blue-800 dark:text-blue-200"><strong>${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.format"))}:</strong> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.formatNotice"))}</div></div></div> <div class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-4"><div class="flex items-start"><span class="text-amber-600 dark:text-amber-400 mr-2">⚠️</span> <div class="text-sm text-amber-800 dark:text-amber-200"><strong>${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.security"))}:</strong> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.securityNotice"))}</div></div></div> <div class="flex flex-col sm:flex-row gap-4 mt-4"><button type="submit"${attr("disabled", !formValid || store_get($$store_subs ??= {}, "$isLoading", isLoading), true)}${attr_class(`flex-1 text-white px-6 py-4 rounded-lg font-semibold border-none transition-all duration-200 flex items-center justify-center ${stringify(!formValid || store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "bg-gray-400 cursor-not-allowed" : "bg-blue-600 hover:bg-blue-700 hover:shadow-lg cursor-pointer")}`)}>`);
  if (store_get($$store_subs ??= {}, "$isLoading", isLoading)) {
    $$payload.out.push("<!--[-->");
    LoadingSpinner($$payload, { size: "sm", class: "mr-2" });
    $$payload.out.push(`<!----> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.loading"))}...`);
  } else {
    $$payload.out.push("<!--[!-->");
    Iconize($$payload, {
      conf: { emoji: "▶", iconSize: "text-lg", spacing: "gap-2" },
      children: ($$payload2) => {
        $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("apiKey.generateApiKey"))}`);
      }
    });
  }
  $$payload.out.push(`<!--]--></button> <button type="button" class="flex-1 bg-gray-600 hover:bg-gray-700 text-white px-6 py-4 rounded-lg font-semibold border-none cursor-pointer hover:shadow-lg transition-all duration-200 flex items-center justify-center gap-2">`);
  Iconize($$payload, {
    conf: { icon: "home", iconSize: "w-5 h-5" },
    children: ($$payload2) => {
      $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.backToMenu"))}`);
    }
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

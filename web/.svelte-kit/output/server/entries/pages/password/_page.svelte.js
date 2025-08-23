import { D as store_get, N as ensure_array_like, J as head, M as escape_html, G as attr, O as maybe_selected, E as attr_class, F as stringify, I as unsubscribe_stores, B as pop, z as push } from "../../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import "../../../chunks/state.svelte.js";
import { p as page, i as isLoading } from "../../../chunks/result.js";
import { L as LoadingSpinner } from "../../../chunks/LoadingSpinner.js";
import { I as Iconize, F as Footer } from "../../../chunks/Footer.js";
import { _ } from "../../../chunks/rtl.js";
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  let alphabetOptions, minLength, lengthValid, seedValid, formValid;
  function getDefaultParams() {
    return {
      length: 21,
      // Minimum for full-with-symbols alphabet
      alphabet: "full-with-symbols",
      raw: true
    };
  }
  let params = getDefaultParams();
  let seedInput = "";
  function isValidHexSeed(seed) {
    return true;
  }
  store_get($$store_subs ??= {}, "$page", page).url.searchParams;
  alphabetOptions = [
    {
      value: "full-with-symbols",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.full-with-symbols"),
      description: store_get($$store_subs ??= {}, "$_", _)("password.maxSecurityDescription")
    },
    {
      value: "no-look-alike",
      label: store_get($$store_subs ??= {}, "$_", _)("alphabets.no-look-alike"),
      description: store_get($$store_subs ??= {}, "$_", _)("password.easyReadDescription")
    }
  ];
  minLength = 21;
  lengthValid = params.length >= minLength && params.length <= 44;
  seedValid = isValidHexSeed();
  formValid = lengthValid && seedValid;
  const each_array = ensure_array_like(alphabetOptions);
  head($$payload, ($$payload2) => {
    $$payload2.title = `<title>${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.title"))}</title>`;
  });
  $$payload.out.push(`<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"><div class="container mx-auto px-4 py-8"><div class="mb-8"><div class="text-center"><div class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4"><span class="text-xl text-white">🔐</span></div> <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.title"))}</h1> <p class="text-gray-600 dark:text-gray-300">${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.description"))}</p></div></div> <div class="max-w-2xl mx-auto"><div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6"><form class="space-y-6"><div><label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.alphabet"))}</label> <select id="alphabet" class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">`);
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
  $$payload.out.push(`<!--]--></div> <div><label for="length" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.length"))} (${escape_html(minLength)}-44 ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.characters"))})</label> <div class="flex items-center gap-4"><input type="range" id="length"${attr("value", params.length)}${attr("min", minLength)} max="44" class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"/> <span class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${escape_html(params.length)}</span></div> `);
  if (!lengthValid) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-red-500 text-sm mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.length"))}
								${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.mustBeBetween"))}
								${escape_html(minLength)}
								${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.and"))} 44</p>`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--> <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mt-3"><p class="text-sm text-blue-800 dark:text-blue-200"><strong>${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.securityNote"))}</strong> `);
  {
    $$payload.out.push("<!--[!-->");
    $$payload.out.push(`${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.fullAlphabetNote").replace("{0}", minLength.toString()))}`);
  }
  $$payload.out.push(`<!--]--></p></div></div> <div><label for="seed" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.optionalSeed"))}</label> <textarea id="seed" maxlength="64" rows="2"${attr("placeholder", store_get($$store_subs ??= {}, "$_", _)("common.optionalSeed"))}${attr_class(`w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white font-mono text-xs resize-none ${stringify(!seedValid ? "border-red-500 focus:ring-red-500 focus:border-red-500" : "")}`)}>`);
  const $$body = escape_html(seedInput);
  if ($$body) {
    $$payload.out.push(`${$$body}`);
  }
  $$payload.out.push(`</textarea> `);
  if (!seedValid) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<p class="text-red-500 text-sm mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.seedInvalid"))}</p>`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--></div> <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"><div class="flex items-start"><span class="text-blue-600 dark:text-blue-400 mr-2">🛡️</span> <div class="text-sm text-blue-800 dark:text-blue-200"><strong>${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.securityNote"))}</strong> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.securityDescription"))}</div></div></div> <div class="flex flex-col sm:flex-row gap-4 mt-4"><button type="submit"${attr("disabled", !formValid || store_get($$store_subs ??= {}, "$isLoading", isLoading), true)}${attr_class(`flex-1 text-white px-6 py-4 rounded-lg font-semibold border-none transition-all duration-200 flex items-center justify-center ${stringify(!formValid || store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "bg-gray-400 cursor-not-allowed" : "bg-blue-600 hover:bg-blue-700 hover:shadow-lg cursor-pointer")}`)}>`);
  if (store_get($$store_subs ??= {}, "$isLoading", isLoading)) {
    $$payload.out.push("<!--[-->");
    LoadingSpinner($$payload, { size: "sm", class: "mr-2" });
    $$payload.out.push(`<!----> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.loading"))}...`);
  } else {
    $$payload.out.push("<!--[!-->");
    Iconize($$payload, {
      conf: { emoji: "▶", iconSize: "text-lg", spacing: "gap-2" },
      children: ($$payload2) => {
        $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("password.generatePassword"))}`);
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

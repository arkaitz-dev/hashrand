import { F as attr_class, E as store_get, N as escape_html, J as unsubscribe_stores, D as pop, A as push, G as stringify, K as head, O as ensure_array_like } from "../../../chunks/index.js";
import { g as goto } from "../../../chunks/client.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import "clsx";
import "../../../chunks/state.svelte.js";
import { I as Icon, i as isRTL, _, c as currentLanguage } from "../../../chunks/rtl.js";
import { F as Footer } from "../../../chunks/Footer.js";
import { B as Button } from "../../../chunks/Button.js";
import { r as resultState, i as isLoading, e as error, s as setLoading, a as setResult, b as setError } from "../../../chunks/result.js";
function BackButton($$payload, $$props) {
  push();
  var $$store_subs;
  let { class: className = "" } = $$props;
  $$payload.out.push(`<button${attr_class(`inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:bg-gray-800 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-700 transition-all duration-200 ${stringify(className)}`)}>`);
  Icon($$payload, {
    name: store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "arrow-right" : "arrow-left",
    size: "w-4 h-4"
  });
  $$payload.out.push(`<!----> ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.back"))}</button>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  let getEndpointDisplayName, formattedTimestamp, translateParameterKey, translateParameterValue;
  let copySuccess = false;
  let copyTimeout;
  async function copyToClipboard() {
    if (!store_get($$store_subs ??= {}, "$resultState", resultState)?.value) return;
    try {
      await navigator.clipboard.writeText(store_get($$store_subs ??= {}, "$resultState", resultState).value);
      copySuccess = true;
      clearTimeout(copyTimeout);
      copyTimeout = setTimeout(
        () => {
          copySuccess = false;
        },
        2e3
      );
    } catch (err) {
      console.error("Failed to copy:", err);
      try {
        const textArea = document.createElement("textarea");
        textArea.value = store_get($$store_subs ??= {}, "$resultState", resultState).value;
        document.body.appendChild(textArea);
        textArea.select();
        document.execCommand("copy");
        document.body.removeChild(textArea);
        copySuccess = true;
        clearTimeout(copyTimeout);
        copyTimeout = setTimeout(
          () => {
            copySuccess = false;
          },
          2e3
        );
      } catch (fallbackErr) {
        console.error("Fallback copy failed:", fallbackErr);
      }
    }
  }
  function getEndpointIcon(endpoint) {
    switch (endpoint) {
      case "custom":
        return "🎲";
      case "generate":
        return "🎲";
      case "password":
        return "🔐";
      case "api-key":
        return "🔑";
      default:
        return "📝";
    }
  }
  function getEndpointColor(endpoint) {
    switch (endpoint) {
      case "custom":
        return "blue";
      case "generate":
        return "blue";
      case "password":
        return "blue";
      case "api-key":
        return "blue";
      default:
        return "gray";
    }
  }
  function getPreviousPath() {
    if (!store_get($$store_subs ??= {}, "$resultState", resultState)) return "/";
    const endpointRoutes = {
      "custom": "/custom",
      "generate": "/custom",
      // backward compatibility
      "password": "/password",
      "api-key": "/api-key"
    };
    return endpointRoutes[store_get($$store_subs ??= {}, "$resultState", resultState).endpoint] || "/";
  }
  async function regenerateHash() {
    if (!store_get($$store_subs ??= {}, "$resultState", resultState) || store_get($$store_subs ??= {}, "$isLoading", isLoading)) return;
    copySuccess = false;
    setLoading(true);
    try {
      const { api } = await import("../../../chunks/api.js");
      let result;
      switch (store_get($$store_subs ??= {}, "$resultState", resultState).endpoint) {
        case "custom":
        case "generate":
          result = await api.generate(store_get($$store_subs ??= {}, "$resultState", resultState).params);
          break;
        case "password":
          result = await api.generatePassword(store_get($$store_subs ??= {}, "$resultState", resultState).params);
          break;
        case "api-key":
          result = await api.generateApiKey(store_get($$store_subs ??= {}, "$resultState", resultState).params);
          break;
        default:
          throw new Error(store_get($$store_subs ??= {}, "$_", _)("common.unknownEndpoint"));
      }
      setResult({
        value: result,
        params: store_get($$store_subs ??= {}, "$resultState", resultState).params,
        endpoint: store_get($$store_subs ??= {}, "$resultState", resultState).endpoint,
        timestamp: /* @__PURE__ */ new Date()
      });
      copySuccess = false;
    } catch (error2) {
      setError(error2 instanceof Error ? error2.message : store_get($$store_subs ??= {}, "$_", _)("common.failedToRegenerate"));
    } finally {
      setLoading(false);
    }
  }
  getEndpointDisplayName = (endpoint) => {
    switch (endpoint) {
      case "custom":
        return store_get($$store_subs ??= {}, "$_", _)("custom.title");
      case "generate":
        return store_get($$store_subs ??= {}, "$_", _)("custom.title");
      case "password":
        return store_get($$store_subs ??= {}, "$_", _)("password.title");
      case "api-key":
        return store_get($$store_subs ??= {}, "$_", _)("apiKey.title");
      default:
        return endpoint;
    }
  };
  formattedTimestamp = store_get($$store_subs ??= {}, "$resultState", resultState)?.timestamp ? (() => {
    const localeMap = {
      "en": "en-US",
      "es": "es-ES",
      "pt": "pt-PT",
      "fr": "fr-FR",
      "de": "de-DE",
      "ru": "ru-RU",
      "zh": "zh-CN",
      "ar": "ar-SA",
      "hi": "hi-IN",
      "ja": "ja-JP",
      "eu": "eu-ES",
      "ca": "ca-ES",
      "gl": "gl-ES"
    };
    const locale = localeMap[store_get($$store_subs ??= {}, "$currentLanguage", currentLanguage)] || "en-US";
    return new Intl.DateTimeFormat(locale, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit"
    }).format(store_get($$store_subs ??= {}, "$resultState", resultState).timestamp);
  })() : "";
  translateParameterKey = (key) => {
    const translations = {
      length: store_get($$store_subs ??= {}, "$_", _)("common.length"),
      alphabet: store_get($$store_subs ??= {}, "$_", _)("common.alphabet"),
      prefix: store_get($$store_subs ??= {}, "$_", _)("custom.prefix") || "Prefix",
      suffix: store_get($$store_subs ??= {}, "$_", _)("custom.suffix") || "Suffix"
    };
    return translations[key] || key.replace(/([A-Z])/g, " $1").trim();
  };
  translateParameterValue = (key, value) => {
    if (typeof value === "boolean") {
      return value ? store_get($$store_subs ??= {}, "$_", _)("common.yes") || "Yes" : store_get($$store_subs ??= {}, "$_", _)("common.no") || "No";
    }
    if (key === "alphabet" && typeof value === "string") {
      return store_get($$store_subs ??= {}, "$_", _)(`alphabets.${value}`) || value;
    }
    return String(value);
  };
  head($$payload, ($$payload2) => {
    $$payload2.title = `<title>${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.result"))}</title>`;
  });
  if (store_get($$store_subs ??= {}, "$resultState", resultState)) {
    $$payload.out.push("<!--[-->");
    const color = getEndpointColor(store_get($$store_subs ??= {}, "$resultState", resultState).endpoint);
    const each_array = ensure_array_like(Object.entries(store_get($$store_subs ??= {}, "$resultState", resultState).params));
    $$payload.out.push(`<div${attr_class(`min-h-screen bg-gradient-to-br from-${stringify(color)}-50 to-${stringify(color)}-100 dark:from-gray-900 dark:to-gray-800`)}><div class="container mx-auto px-4 py-8"><div class="mb-8"><div class="text-center"><div${attr_class(`inline-flex items-center justify-center w-16 h-16 bg-${stringify(color)}-600 rounded-full mb-6`)}><span class="text-2xl text-white">${escape_html(getEndpointIcon(store_get($$store_subs ??= {}, "$resultState", resultState).endpoint))}</span></div> <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.result"))}</h1></div></div> <div class="max-w-4xl mx-auto"><div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6 mb-6"><div class="mb-6"><label for="generated-value" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.generatedValue"))}</label> <div class="relative"><textarea id="generated-value" readonly${attr_class(`w-full p-4 pb-12 bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-lg font-mono text-sm resize-none focus:ring-2 focus:ring-${stringify(color)}-500 focus:border-${stringify(color)}-500 min-h-[100px] ${stringify(store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "text-gray-500 dark:text-gray-400" : "")}`)}>`);
    const $$body = escape_html(store_get($$store_subs ??= {}, "$isLoading", isLoading) ? store_get($$store_subs ??= {}, "$_", _)("common.loading") + "..." : store_get($$store_subs ??= {}, "$resultState", resultState).value);
    if ($$body) {
      $$payload.out.push(`${$$body}`);
    }
    $$payload.out.push(`</textarea> `);
    if (!store_get($$store_subs ??= {}, "$isLoading", isLoading)) {
      $$payload.out.push("<!--[-->");
      Button($$payload, {
        onclick: copyToClipboard,
        class: `absolute bottom-3 ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "left-3" : "right-3")} px-2 py-1 text-xs font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-1 ${stringify(copySuccess ? "bg-green-600 hover:bg-green-700 text-white" : "bg-blue-600 hover:bg-blue-700 text-white")}`,
        icon: copySuccess ? "check" : "copy",
        iconSize: "w-3 h-3",
        children: ($$payload2) => {
          $$payload2.out.push(`<!---->${escape_html(copySuccess ? store_get($$store_subs ??= {}, "$_", _)("common.copied") : store_get($$store_subs ??= {}, "$_", _)("common.copy"))}`);
        },
        $$slots: { default: true }
      });
    } else {
      $$payload.out.push("<!--[!-->");
    }
    $$payload.out.push(`<!--]--></div></div> <div class="grid grid-cols-1 md:grid-cols-2 gap-6"><div><button class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"><h3 class="text-lg font-semibold text-gray-900 dark:text-white">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.generationDetails"))}</h3> `);
    Icon($$payload, {
      name: "chevron-down",
      size: "w-5 h-5",
      class: `text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 ${stringify("")} ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "rtl-flip-chevron" : "")}`
    });
    $$payload.out.push(`<!----></button> <dl${attr_class(`space-y-2 ${stringify("hidden")} md:block`)}><div><dt class="text-sm font-medium text-gray-500 dark:text-gray-400">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.type"))}</dt> <dd class="text-sm text-gray-900 dark:text-white">${escape_html(getEndpointDisplayName(store_get($$store_subs ??= {}, "$resultState", resultState).endpoint))}</dd></div> <div><dt class="text-sm font-medium text-gray-500 dark:text-gray-400">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.length"))}</dt> <dd class="text-sm text-gray-900 dark:text-white">${escape_html(store_get($$store_subs ??= {}, "$resultState", resultState).value.length)} ${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.characters"))}</dd></div> <div><dt class="text-sm font-medium text-gray-500 dark:text-gray-400">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.generated"))}</dt> <dd class="text-sm text-gray-900 dark:text-white">${escape_html(formattedTimestamp)}</dd></div></dl></div> <div><button class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"><h3 class="text-lg font-semibold text-gray-900 dark:text-white">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.parametersUsed"))}</h3> `);
    Icon($$payload, {
      name: "chevron-down",
      size: "w-5 h-5",
      class: `text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 ${stringify("")} ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "rtl-flip-chevron" : "")}`
    });
    $$payload.out.push(`<!----></button> <dl${attr_class(`space-y-2 ${stringify("hidden")} md:block`)}><!--[-->`);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let [key, value] = each_array[$$index];
      if (value !== void 0 && value !== null && value !== "" && key !== "raw") {
        $$payload.out.push("<!--[-->");
        $$payload.out.push(`<div><dt class="text-sm font-medium text-gray-500 dark:text-gray-400 capitalize">${escape_html(translateParameterKey(key))}</dt> <dd class="text-sm text-gray-900 dark:text-white">${escape_html(translateParameterValue(key, value))}</dd></div>`);
      } else {
        $$payload.out.push("<!--[!-->");
      }
      $$payload.out.push(`<!--]-->`);
    }
    $$payload.out.push(`<!--]--></dl></div></div></div> <div class="flex flex-col sm:flex-row gap-4 justify-center">`);
    Button($$payload, {
      onclick: regenerateHash,
      disabled: store_get($$store_subs ??= {}, "$isLoading", isLoading),
      class: `px-6 py-3 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2 min-w-[180px] ${stringify(store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "bg-gray-400 cursor-not-allowed" : "bg-blue-600 hover:bg-blue-700 cursor-pointer")}`,
      icon: "refresh",
      iconClass: store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "animate-spin-fast" : "",
      children: ($$payload2) => {
        $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.generateAnother"))}`);
      },
      $$slots: { default: true }
    });
    $$payload.out.push(`<!----> `);
    Button($$payload, {
      onclick: () => goto(getPreviousPath()),
      class: "px-6 py-3 bg-gray-500 hover:bg-gray-600 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2 min-w-[180px]",
      icon: "settings",
      children: ($$payload2) => {
        $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.adjustSettings"))}`);
      },
      $$slots: { default: true }
    });
    $$payload.out.push(`<!----> `);
    Button($$payload, {
      onclick: () => goto(),
      class: "px-6 py-3 bg-gray-600 hover:bg-gray-700 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2 min-w-[180px]",
      icon: "briefcase",
      children: ($$payload2) => {
        $$payload2.out.push(`<!---->${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.backToMenu"))}`);
      },
      $$slots: { default: true }
    });
    $$payload.out.push(`<!----></div></div> `);
    Footer($$payload);
    $$payload.out.push(`<!----></div></div>`);
  } else {
    $$payload.out.push("<!--[!-->");
    if (store_get($$store_subs ??= {}, "$error", error)) {
      $$payload.out.push("<!--[-->");
      $$payload.out.push(`<div class="min-h-screen bg-gradient-to-br from-red-50 to-red-100 dark:from-gray-900 dark:to-gray-800"><div class="container mx-auto px-4 py-8"><div class="max-w-2xl mx-auto text-center"><div class="inline-flex items-center justify-center w-16 h-16 bg-red-600 rounded-full mb-6"><span class="text-2xl text-white">❌</span></div> <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">${escape_html(store_get($$store_subs ??= {}, "$_", _)("common.error"))}</h1> <p class="text-gray-600 dark:text-gray-300 mb-8">${escape_html(store_get($$store_subs ??= {}, "$error", error))}</p> `);
      BackButton($$payload, {});
      $$payload.out.push(`<!----></div> `);
      Footer($$payload);
      $$payload.out.push(`<!----></div></div>`);
    } else {
      $$payload.out.push("<!--[!-->");
    }
    $$payload.out.push(`<!--]-->`);
  }
  $$payload.out.push(`<!--]-->`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  _page as default
};

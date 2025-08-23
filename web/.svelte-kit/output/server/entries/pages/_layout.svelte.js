import { E as store_get, F as attr_class, G as stringify, I as attr, J as unsubscribe_stores, D as pop, A as push, K as head } from "../../chunks/index.js";
import "@sveltejs/kit/internal";
import { w as writable } from "../../chunks/exports.js";
import "../../chunks/utils.js";
import "../../chunks/state.svelte.js";
import { I as Icon, c as currentLanguage, i as isRTL } from "../../chunks/rtl.js";
const favicon = "data:image/svg+xml,%3csvg%20xmlns='http://www.w3.org/2000/svg'%20width='107'%20height='128'%20viewBox='0%200%20107%20128'%3e%3ctitle%3esvelte-logo%3c/title%3e%3cpath%20d='M94.157%2022.819c-10.4-14.885-30.94-19.297-45.792-9.835L22.282%2029.608A29.92%2029.92%200%200%200%208.764%2049.65a31.5%2031.5%200%200%200%203.108%2020.231%2030%2030%200%200%200-4.477%2011.183%2031.9%2031.9%200%200%200%205.448%2024.116c10.402%2014.887%2030.942%2019.297%2045.791%209.835l26.083-16.624A29.92%2029.92%200%200%200%2098.235%2078.35a31.53%2031.53%200%200%200-3.105-20.232%2030%2030%200%200%200%204.474-11.182%2031.88%2031.88%200%200%200-5.447-24.116'%20style='fill:%23ff3e00'/%3e%3cpath%20d='M45.817%20106.582a20.72%2020.72%200%200%201-22.237-8.243%2019.17%2019.17%200%200%201-3.277-14.503%2018%2018%200%200%201%20.624-2.435l.49-1.498%201.337.981a33.6%2033.6%200%200%200%2010.203%205.098l.97.294-.09.968a5.85%205.85%200%200%200%201.052%203.878%206.24%206.24%200%200%200%206.695%202.485%205.8%205.8%200%200%200%201.603-.704L69.27%2076.28a5.43%205.43%200%200%200%202.45-3.631%205.8%205.8%200%200%200-.987-4.371%206.24%206.24%200%200%200-6.698-2.487%205.7%205.7%200%200%200-1.6.704l-9.953%206.345a19%2019%200%200%201-5.296%202.326%2020.72%2020.72%200%200%201-22.237-8.243%2019.17%2019.17%200%200%201-3.277-14.502%2017.99%2017.99%200%200%201%208.13-12.052l26.081-16.623a19%2019%200%200%201%205.3-2.329%2020.72%2020.72%200%200%201%2022.237%208.243%2019.17%2019.17%200%200%201%203.277%2014.503%2018%2018%200%200%201-.624%202.435l-.49%201.498-1.337-.98a33.6%2033.6%200%200%200-10.203-5.1l-.97-.294.09-.968a5.86%205.86%200%200%200-1.052-3.878%206.24%206.24%200%200%200-6.696-2.485%205.8%205.8%200%200%200-1.602.704L37.73%2051.72a5.42%205.42%200%200%200-2.449%203.63%205.79%205.79%200%200%200%20.986%204.372%206.24%206.24%200%200%200%206.698%202.486%205.8%205.8%200%200%200%201.602-.704l9.952-6.342a19%2019%200%200%201%205.295-2.328%2020.72%2020.72%200%200%201%2022.237%208.242%2019.17%2019.17%200%200%201%203.277%2014.503%2018%2018%200%200%201-8.13%2012.053l-26.081%2016.622a19%2019%200%200%201-5.3%202.328'%20style='fill:%23fff'/%3e%3c/svg%3e";
function getInitialTheme() {
  return "dark";
}
const theme = writable(getInitialTheme());
function TopControls($$payload, $$props) {
  push();
  var $$store_subs;
  let showDropdown = false;
  const languages = [
    { code: "ar", name: "العربية", flag: "saudi" },
    // Arabiya
    { code: "ca", name: "Català", flag: "catalonia" },
    // Catala  
    { code: "de", name: "Deutsch", flag: "germany" },
    // Deutsch
    { code: "en", name: "English", flag: "uk" },
    // English
    { code: "es", name: "Español", flag: "spain" },
    // Espanol
    { code: "eu", name: "Euskera", flag: "basque" },
    // Euskera
    { code: "fr", name: "Français", flag: "france" },
    // Francais
    { code: "gl", name: "Galego", flag: "galicia" },
    // Galego
    { code: "hi", name: "हिंदी", flag: "india" },
    // Hindi
    { code: "ja", name: "日本語", flag: "japan" },
    // Nihongo
    { code: "pt", name: "Português", flag: "portugal" },
    // Portugues
    { code: "ru", name: "Русский", flag: "russia" },
    // Russkiy
    { code: "zh", name: "中文", flag: "china" }
    // Zhongwen
  ];
  function findLanguageByCode(code) {
    return languages.find((lang) => lang.code === code) || languages[0];
  }
  let selectedLanguage = findLanguageByCode(store_get($$store_subs ??= {}, "$currentLanguage", currentLanguage));
  store_get($$store_subs ??= {}, "$isRTL", isRTL);
  $$payload.out.push(`<div${attr_class(`top-controls absolute top-0.5 md:top-4 z-50 flex items-center gap-1 bg-gray-200/90 dark:bg-gray-800/80 backdrop-blur-sm rounded-2xl p-1 md:p-1 shadow-lg border border-gray-400/50 dark:border-gray-700/50 transition-opacity duration-[1500ms] ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "left-0.5 md:left-4" : "right-0.5 md:right-4")} ${stringify("opacity-100")}`)}><div class="relative"><button${attr_class("p-2 rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-colors duration-[750ms] transition-shadow duration-[750ms] transition-border-colors duration-[750ms] transform hover:scale-105 focus:outline-none flex items-center justify-center w-12 h-12", void 0, {
    "bg-white": showDropdown,
    "dark:bg-gray-800": showDropdown,
    "shadow-lg": showDropdown,
    "border-gray-200": showDropdown,
    "dark:border-gray-700": showDropdown,
    "scale-105": showDropdown
  })} aria-label="Select language">`);
  Icon($$payload, { name: selectedLanguage.flag, size: "w-12 h-12" });
  $$payload.out.push(`<!----></button> `);
  {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--></div> <button class="p-2 rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-colors duration-[750ms] transition-shadow duration-[750ms] transition-border-colors duration-[750ms] transform hover:scale-105 focus:outline-none flex items-center justify-center w-12 h-12"${attr("aria-label", store_get($$store_subs ??= {}, "$theme", theme) === "dark" ? "Switch to light mode" : "Switch to dark mode")}${attr("title", store_get($$store_subs ??= {}, "$theme", theme) === "dark" ? "Switch to light mode" : "Switch to dark mode")}><div class="text-gray-700 dark:text-gray-300 transition-all duration-150 transform">`);
  if (store_get($$store_subs ??= {}, "$theme", theme) === "dark") {
    $$payload.out.push("<!--[-->");
    Icon($$payload, { name: "moon", size: "w-5 h-5" });
  } else {
    $$payload.out.push("<!--[!-->");
    Icon($$payload, { name: "sun", size: "w-5 h-5" });
  }
  $$payload.out.push(`<!--]--></div></button></div>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
function _layout($$payload, $$props) {
  push();
  let { children } = $$props;
  head($$payload, ($$payload2) => {
    $$payload2.out.push(`<link rel="icon"${attr("href", favicon)}/> <meta name="viewport" content="width=device-width, initial-scale=1.0"/> <meta name="theme-color" content="#3b82f6" media="(prefers-color-scheme: light)"/> <meta name="theme-color" content="#1e293b" media="(prefers-color-scheme: dark)"/>`);
  });
  $$payload.out.push(`<main class="min-h-screen relative">`);
  TopControls($$payload);
  $$payload.out.push(`<!----> `);
  children?.($$payload);
  $$payload.out.push(`<!----></main>`);
  pop();
}
export {
  _layout as default
};

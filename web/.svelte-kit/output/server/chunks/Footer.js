import { D as store_get, E as attr_class, M as escape_html, I as unsubscribe_stores, B as pop, z as push, F as stringify } from "./index.js";
import { I as Icon, i as isRTL, _ } from "./rtl.js";
function Iconize($$payload, $$props) {
  push();
  var $$store_subs;
  let { conf, class: wrapperClass = "", children } = $$props;
  const currentIcon = store_get($$store_subs ??= {}, "$isRTL", isRTL) && conf.rtlIcon ? conf.rtlIcon : conf.icon;
  const currentEmoji = store_get($$store_subs ??= {}, "$isRTL", isRTL) && conf.rtlEmoji ? conf.rtlEmoji : conf.emoji;
  const useEmoji = !!currentEmoji;
  const iconSize = conf.iconSize || (currentEmoji ? "text-3xl" : "w-4 h-4");
  const spacing = conf.spacing || "gap-2";
  const invertposition = conf.invertposition || false;
  function getIconPlaceholder(iconName) {
    if (iconName === "arrow-left" || iconName === "arrow-right") {
      return ">";
    }
    return "auto";
  }
  $$payload.out.push(`<div${attr_class(`inline-flex items-center ${stringify(spacing)} ${stringify(wrapperClass)}`)}>`);
  if (invertposition) {
    $$payload.out.push("<!--[-->");
    children?.($$payload);
    $$payload.out.push(`<!---->`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--> `);
  if (useEmoji) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<span${attr_class(`inline-block ${stringify(iconSize)}`)} style="direction: ltr;">${escape_html(currentEmoji)}</span>`);
  } else {
    $$payload.out.push("<!--[!-->");
    if (currentIcon) {
      $$payload.out.push("<!--[-->");
      Icon($$payload, {
        name: currentIcon,
        size: iconSize,
        placeholder: getIconPlaceholder(currentIcon)
      });
    } else {
      $$payload.out.push("<!--[!-->");
    }
    $$payload.out.push(`<!--]-->`);
  }
  $$payload.out.push(`<!--]--> `);
  if (!invertposition) {
    $$payload.out.push("<!--[-->");
    children?.($$payload);
    $$payload.out.push(`<!---->`);
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--></div>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
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
  Icon($$payload, {
    name: "heart",
    size: "w-3 h-3 mx-1 text-red-500",
    placeholder: "auto"
  });
  $$payload.out.push(`<!----> <span>by</span> <a href="https://arkaitz.dev" target="_blank" rel="noopener noreferrer" class="ml-1 text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 hover:underline">Arkaitz Dev</a></div></div>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  Footer as F,
  Iconize as I
};

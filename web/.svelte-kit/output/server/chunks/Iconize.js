import { E as store_get, F as attr_class, Q as clsx, N as escape_html, R as slot, J as unsubscribe_stores, D as pop, A as push, G as stringify } from "./index.js";
import { I as Icon, i as isRTL } from "./rtl.js";
function Iconize($$payload, $$props) {
  push();
  var $$store_subs;
  let { conf, class: wrapperClass = "" } = $$props;
  const config = {
    iconClass: "",
    iconSize: conf.emoji ? "text-3xl" : "w-4 h-4",
    // Different defaults for emoji vs icon
    rtlIcon: conf.icon,
    // Use same icon for RTL by default
    rtlEmoji: conf.emoji,
    // Use same emoji for RTL by default
    rtlIconClass: conf.iconClass || "",
    spacing: "gap-2",
    rtlAware: true,
    ...conf
  };
  const useEmoji = !!config.emoji;
  const currentIcon = store_get($$store_subs ??= {}, "$isRTL", isRTL) && config.rtlAware ? config.rtlIcon : config.icon;
  const currentEmoji = store_get($$store_subs ??= {}, "$isRTL", isRTL) && config.rtlAware ? config.rtlEmoji : config.emoji;
  const currentIconClass = store_get($$store_subs ??= {}, "$isRTL", isRTL) && config.rtlAware ? config.rtlIconClass : config.iconClass;
  const wrapperClasses = `inline-flex items-center ${config.spacing} ${wrapperClass}`;
  $$payload.out.push(`<span class="inline-block"><span${attr_class(clsx(wrapperClasses))}>`);
  if (useEmoji) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<span${attr_class(`inline-block ${stringify(config.iconSize)} ${stringify(currentIconClass)}`)} style="direction: ltr;">${escape_html(currentEmoji)}</span>`);
  } else {
    $$payload.out.push("<!--[!-->");
    if (currentIcon) {
      $$payload.out.push("<!--[-->");
      Icon($$payload, {
        name: currentIcon,
        size: config.iconSize,
        class: currentIconClass
      });
    } else {
      $$payload.out.push("<!--[!-->");
    }
    $$payload.out.push(`<!--]-->`);
  }
  $$payload.out.push(`<!--]--> <span><!---->`);
  slot($$payload, $$props, "default", {});
  $$payload.out.push(`<!----></span></span></span>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  Iconize as I
};

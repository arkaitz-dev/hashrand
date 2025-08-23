import { S as sanitize_props, T as rest_props, U as fallback, V as spread_attributes, G as stringify, E as store_get, Q as clsx, R as slot, J as unsubscribe_stores, W as bind_props, D as pop, A as push } from "./index.js";
import { I as Icon, i as isRTL } from "./rtl.js";
function Button($$payload, $$props) {
  const $$sanitized_props = sanitize_props($$props);
  const $$restProps = rest_props($$sanitized_props, ["type", "disabled", "icon", "iconClass", "rtlAware"]);
  push();
  var $$store_subs;
  let type = fallback($$props["type"], "button");
  let disabled = fallback($$props["disabled"], false);
  let icon = fallback($$props["icon"], void 0);
  let iconClass = fallback($$props["iconClass"], "");
  let rtlAware = fallback(
    $$props["rtlAware"],
    true
    // Por defecto siempre RTL-aware
  );
  $$payload.out.push(`<button${spread_attributes(
    {
      type,
      disabled,
      class: clsx($$sanitized_props.class || ""),
      style: `direction: ${stringify(store_get($$store_subs ??= {}, "$isRTL", isRTL) ? "rtl" : "ltr")}`,
      ...$$restProps
    }
  )}>`);
  if (icon) {
    $$payload.out.push("<!--[-->");
    Icon($$payload, {
      name: icon,
      size: $$sanitized_props.iconSize || "w-4 h-4",
      class: iconClass
    });
  } else {
    $$payload.out.push("<!--[!-->");
  }
  $$payload.out.push(`<!--]--> <!---->`);
  slot($$payload, $$props, "default", {});
  $$payload.out.push(`<!----></button>`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  bind_props($$props, { type, disabled, icon, iconClass, rtlAware });
  pop();
}
export {
  Button as B
};

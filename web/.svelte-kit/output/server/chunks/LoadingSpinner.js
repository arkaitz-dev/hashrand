import { E as attr_class, F as stringify } from "./index.js";
import { I as Icon } from "./rtl.js";
function LoadingSpinner($$payload, $$props) {
  let { size = "md", class: className = "" } = $$props;
  const sizeClasses = { sm: "w-4 h-4", md: "w-6 h-6", lg: "w-8 h-8" };
  $$payload.out.push(`<div${attr_class(`inline-flex items-center justify-center ${stringify(className)}`)}>`);
  Icon($$payload, {
    name: "spinner",
    size: `${stringify(sizeClasses[size])} animate-spin`,
    class: "text-blue-600 dark:text-blue-400"
  });
  $$payload.out.push(`<!----></div>`);
}
export {
  LoadingSpinner as L
};

import { K as getContext } from "./index.js";
import "@sveltejs/kit/internal";
import { w as writable } from "./exports.js";
import "./utils.js";
import "clsx";
import "./state.svelte.js";
const getStores = () => {
  const stores$1 = getContext("__svelte__");
  return {
    /** @type {typeof page} */
    page: {
      subscribe: stores$1.page.subscribe
    },
    /** @type {typeof navigating} */
    navigating: {
      subscribe: stores$1.navigating.subscribe
    },
    /** @type {typeof updated} */
    updated: stores$1.updated
  };
};
const page = {
  subscribe(fn) {
    const store = getStores().page;
    return store.subscribe(fn);
  }
};
const resultState = writable(null);
const isLoading = writable(false);
const error = writable(null);
export {
  error as e,
  isLoading as i,
  page as p,
  resultState as r
};

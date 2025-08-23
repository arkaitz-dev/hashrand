import { w as writable } from "./exports.js";
const resultState = writable(null);
const isLoading = writable(false);
const error = writable(null);
export {
  error as e,
  isLoading as i,
  resultState as r
};

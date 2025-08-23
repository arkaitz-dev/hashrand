import { w as writable } from "./exports.js";
const resultState = writable(null);
const isLoading = writable(false);
const error = writable(null);
function setResult(result) {
  resultState.set(result);
  error.set(null);
}
function setError(errorMessage) {
  error.set(errorMessage);
  isLoading.set(false);
}
function setLoading(loading) {
  isLoading.set(loading);
  if (loading) {
    error.set(null);
  }
}
export {
  setResult as a,
  setError as b,
  error as e,
  isLoading as i,
  resultState as r,
  setLoading as s
};

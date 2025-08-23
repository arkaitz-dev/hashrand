const API_BASE = "/api";
class ApiError extends Error {
  constructor(message, status) {
    super(message);
    this.status = status;
    this.name = "ApiError";
  }
}
async function handleResponse(response) {
  if (!response.ok) {
    const errorText = await response.text();
    throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
  }
  return response.text();
}
async function handleJsonResponse(response) {
  if (!response.ok) {
    const errorText = await response.text();
    throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
  }
  return response.json();
}
const api = {
  async generate(params) {
    const searchParams = new URLSearchParams();
    if (params.length !== void 0) searchParams.set("length", params.length.toString());
    if (params.alphabet) searchParams.set("alphabet", params.alphabet);
    if (params.prefix) searchParams.set("prefix", params.prefix);
    if (params.suffix) searchParams.set("suffix", params.suffix);
    if (params.raw) searchParams.set("raw", "true");
    const response = await fetch(`${API_BASE}/generate?${searchParams}`);
    return handleResponse(response);
  },
  async generatePassword(params) {
    const searchParams = new URLSearchParams();
    if (params.length !== void 0) searchParams.set("length", params.length.toString());
    if (params.alphabet) searchParams.set("alphabet", params.alphabet);
    if (params.raw) searchParams.set("raw", "true");
    const response = await fetch(`${API_BASE}/password?${searchParams}`);
    return handleResponse(response);
  },
  async generateApiKey(params) {
    const searchParams = new URLSearchParams();
    if (params.length !== void 0) searchParams.set("length", params.length.toString());
    if (params.alphabet) searchParams.set("alphabet", params.alphabet);
    if (params.raw) searchParams.set("raw", "true");
    const response = await fetch(`${API_BASE}/api-key?${searchParams}`);
    return handleResponse(response);
  },
  async getVersion() {
    const response = await fetch(`${API_BASE}/version`);
    return handleJsonResponse(response);
  }
};
export {
  ApiError,
  api
};

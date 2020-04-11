import config from "next/config";
import axios from "axios";
const { API_ENDPOINT } = config().publicRuntimeConfig;

export const setAuthorization = (): void => {
  if (typeof window === "undefined") return;

  const token = window.localStorage.getItem("token");
  if (token) {
    // eslint-disable-next-line @typescript-eslint/no-use-before-define
    instance.defaults.headers.common["Authorization"] = `Bearer ${token}`;
  }
};

export const setToken = (token: string): void => {
  window.localStorage.setItem("token", token);
};

export const isLogged = (): boolean => {
  return !!window.localStorage.getItem("token");
};

const instance = axios.create({
  baseURL: API_ENDPOINT,
  headers: {
    "Content-Type": "application/json"
  },
  // In milliseconds
  timeout: 10000
});

export default instance;

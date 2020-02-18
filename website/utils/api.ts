import config from "next/config";
import axios from "axios";
const { API_URL } = config().publicRuntimeConfig;

export const setAuthorization = () => {
  if (typeof window === "undefined") return;
  const token = window.localStorage.getItem("token");
  if (!!token) {
    instance.defaults.headers.common["Authorization"] = `Bearer ${token}`;
  }
};

export const setToken = (token: string) => {
  window.localStorage.setItem("token", token);
};

export const isLogged = (): boolean => {
  return !!window.localStorage.getItem("token");
};

const instance = axios.create({
  baseURL: API_URL,
  // withCredentials: true,
  headers: {
    "Content-Type": "application/json"
  },
  // In milliseconds
  timeout: 10000
});

export default instance;

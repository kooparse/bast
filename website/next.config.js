/* eslint-disable */

const isDev = process.env.NODE_ENV !== "production";

const API_URL = isDev
  ? "http://localhost:3333/api"
  : `/api`;

const SCRIPT_URL = isDev
  ? "http://localhost:3333/script.js"
  : `/script.js`;

module.exports = {
  env: {},
  publicRuntimeConfig: {
    API_URL,
    SCRIPT_URL
  }
};

/* eslint-disable */

const isDev = process.env.NODE_ENV !== "production";

const API_URL = isDev
  ? "http://localhost:3333/api"
  : `${process.env.APP_ADDRESS}/api`;

const SCRIPT_URL = isDev
  ? "http://localhost:3333/script.js"
  : `${process.env.APP_ADDRESS}/script.js`;

module.exports = {
  env: {},
  publicRuntimeConfig: {
    API_URL,
    SCRIPT_URL
  }
};

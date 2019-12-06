const isDev = process.env.NODE_ENV !== "production";
const API_URL = isDev
  ? "http://localhost:3333/api"
  : "https://test123342.herokuapp.com/api";

const SCRIPT_URL = isDev
  ? "http://localhost:3333/script.js"
  : "https://test123342.herokuapp.com/script.js";

module.exports = {
  outDir: "./build",
  env: {},
  publicRuntimeConfig: {
    API_URL,
    SCRIPT_URL,
  }
};

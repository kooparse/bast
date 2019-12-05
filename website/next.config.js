const isDev = process.env.NODE_ENV !== "production";
const API_URL = isDev
  ? "http://localhost:3333/api"
  : "https://test123342.herokuapp.com/api";

module.exports = {
  outDir: "./build",
  env: {},
  publicRuntimeConfig: {
    API_URL
  }
};

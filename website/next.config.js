/* eslint-disable */

const isDev = process.env.NODE_ENV !== "production";

const API_ENDPOINT = isDev ? "http://localhost:3333/api" : `/api`;

const SCRIPT_ENDPOINT = isDev
  ? "http://localhost:3333/script.js"
  : `/script.js`;

const COLLECT_GHOST_PICTURE = isDev
  ? "http://localhost:3333/ghost.png"
  : `/ghost.png`;

module.exports = {
  env: {},
  publicRuntimeConfig: {
    API_ENDPOINT,
    SCRIPT_ENDPOINT,
    COLLECT_GHOST_PICTURE,
  },
};

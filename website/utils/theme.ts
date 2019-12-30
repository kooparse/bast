import { createGlobalStyle } from "styled-components";

export const GlobalStyles = createGlobalStyle`
  body {
    margin: 0;
  }
`;

export const customTheme = {
  global: {
    font: {
      family: "Helvetica",
      size: "14px",
      height: "20px"
    }
  }
};

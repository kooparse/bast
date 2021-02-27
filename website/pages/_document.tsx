import React, { ReactElement } from "react";
import { ColorModeScript } from "@chakra-ui/react";
import Document, { Html, Head, Main, NextScript } from "next/document";
import theme from "../utils/theme";

class Bast extends Document {
  render(): ReactElement {
    return (
      <Html>
        <Head>
          <meta
            name="description"
            content="Minimal and simple website analytics"
          />
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <body>
          <ColorModeScript initialColorMode={theme.config.initialColorMode} />
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default Bast;

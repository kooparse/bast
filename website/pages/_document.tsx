import React, { ReactElement } from "react";
import Document, { Html, Head, Main, NextScript } from "next/document";

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
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default Bast;

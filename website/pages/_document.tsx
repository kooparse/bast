import Document, { Html, Head, Main, NextScript } from "next/document";

class Bast extends Document {
  render() {
    return (
      <Html>
        <Head>
          <meta
            name="description"
            content="The hidden place where knowledge is shared."
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

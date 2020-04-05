import React, { ReactElement } from "react";
import App, { AppContext, AppInitialProps } from "next/app";
import cookies from "next-cookies";
import NavBar from "../components/NavBar";
import {
  ThemeProvider,
  CSSReset,
  ColorModeProvider,
  Box,
  Flex,
  theme
} from "@chakra-ui/core";
import api, { setAuthorization, isLogged } from "../utils/api";
import { UserContext } from "../utils/context";

type InitialProps = AppInitialProps & { initialColorMode: "light" | "dark" };

export default class Website extends App<InitialProps, { user: User }> {
  state = { user: null };

  constructor(props) {
    super(props);
    setAuthorization();
  }

  setUser = (user: User): void => {
    this.setState({ user });
  };

  static async getInitialProps({
    Component,
    ctx
  }: AppContext): Promise<InitialProps> {
    let pageProps = {};

    if (Component.getInitialProps) {
      pageProps = await Component.getInitialProps(ctx);
    }

    // TODO: Remove this when issue on chakra-ui is fixed.
    // See https://github.com/chakra-ui/chakra-ui/issues/349.
    const { isDarkMode = "false" } = cookies(ctx);
    return {
      pageProps,
      initialColorMode: isDarkMode === "true" ? "dark" : "light"
    };
  }

  async componentDidMount(): Promise<void> {
    if (isLogged()) {
      try {
        const { data: user } = await api.get("/user");
        this.setState({ user });
      } catch (err) {
        console.error(err);
      }
    }
  }

  render(): ReactElement {
    const { Component, pageProps, initialColorMode } = this.props;

    return (
      <UserContext.Provider
        value={{ user: this.state.user, setUser: this.setUser }}
      >
        <ThemeProvider theme={theme}>
          <ColorModeProvider value={initialColorMode}>
            <CSSReset />
            <NavBar />
            <Flex width={800} mt="40px" mb="40px" mr="auto" ml="auto">
              <Box width="100%">
                <Component {...pageProps} />
              </Box>
            </Flex>
            <Box margin="25px"></Box>
          </ColorModeProvider>
        </ThemeProvider>
      </UserContext.Provider>
    );
  }
}

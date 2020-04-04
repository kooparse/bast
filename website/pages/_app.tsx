import React, { ReactElement } from "react";
import App from "next/app";
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

export default class Website extends App<{}, { user: User }> {
  state = { user: null };

  constructor(props) {
    super(props);
    setAuthorization();
  }

  setUser = (user: User): void => {
    this.setState({ user });
  };

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
    const { Component, pageProps } = this.props;

    return (
      <UserContext.Provider
        value={{ user: this.state.user, setUser: this.setUser }}
      >
        <ThemeProvider theme={theme}>
          <ColorModeProvider>
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

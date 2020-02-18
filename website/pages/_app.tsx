import React from "react";
import Head from "next/head";
import App from "next/app";
import NavBar from "../components/NavBar";
import { Grommet, Main, Grid } from "grommet";
import { grommet, dark } from "grommet/themes";
import { Home } from "grommet-icons";
import { ThemeProvider, CSSReset, theme } from "@chakra-ui/core";
import api, { setAuthorization, isLogged } from "../utils/api";
import { UserContext } from "../utils/context";
import { GlobalStyles, customTheme } from "../utils/theme";

export default class Website extends App<{}, { user: User }> {
  state = { user: {} };

  constructor(props) {
    super(props);
    setAuthorization();
  }

  setUser = (user: User) => {
    this.setState({ user });
  };

  async componentDidMount() {
    if (isLogged()) {
      const { data: user } = await api.get("/user");
      this.setState({ user });
    }
  }

  render() {
    const { Component, pageProps } = this.props;

    return (
      <UserContext.Provider
        value={{ user: this.state.user, setUser: this.setUser }}
      >
        <GlobalStyles />
        <ThemeProvider theme={theme}>
          <CSSReset />
          <Grommet theme={customTheme}>
            <NavBar />
            <Main as="main" pad="small">
              <Grid columns={["xlarge"]} justifyContent="center">
                <Component {...pageProps} />
              </Grid>
            </Main>
          </Grommet>
        </ThemeProvider>
      </UserContext.Provider>
    );
  }
}

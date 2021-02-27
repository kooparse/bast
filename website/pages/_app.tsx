import React, { ReactElement } from "react";
import App, { AppInitialProps } from "next/app";
import NavBar from "../components/NavBar";
import { CSSReset, Box, Flex, ChakraProvider } from "@chakra-ui/react";
import api, { setAuthorization, isLogged } from "../utils/api";
import { UserContext } from "../utils/context";
import theme from "../utils/theme"

type InitialProps = AppInitialProps & { initialColorMode: "light" | "dark" };

type State = {
  user?: User;
  loading: boolean;
};

export default class Website extends App<InitialProps, { user: User }> {
  state: State = {
    user: null,
    loading: true,
  };

  constructor(props: any) {
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
    this.setState({ loading: false });
  }

  render(): ReactElement {
    const { user, loading } = this.state;
    const { Component, pageProps } = this.props;

    return (
      <UserContext.Provider value={{ user, setUser: this.setUser, loading }}>
        <ChakraProvider theme={theme}>
          <CSSReset />
          <NavBar />
          <Flex maxWidth={800} mt="40px" mb="40px" mx="auto" px={5}>
            <Box width="100%">
              <Component {...pageProps} />
            </Box>
          </Flex>
          <Box margin="25px"></Box>
        </ChakraProvider>
      </UserContext.Provider>
    );
  }
}

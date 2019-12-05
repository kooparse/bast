import React from "react";
import Head from "next/head";
import App from "next/app";
import styled from "styled-components";
import NavBar from "../components/NavBar";
import { createGlobalStyle } from "styled-components";
import api, { setAuthorization, isLogged } from "../utils/api";
import { UserContext } from "../utils/context";

const GlobalStyle = createGlobalStyle`
	body {
    font-family: 'Lato', sans-serif;
		color: #ddd;
		background-color: #1b1b1b;
	}
`;

type User = {
  username: string;
  id: number;
};

interface IState {
  user: User;
}

class Website extends App<{}, IState> {
  state = { user: {} };

  constructor(props) {
    super(props);
    setAuthorization();
  }

  setUser = (user: User) => {
    this.setState({ user });
  };

  async componentDidMount() {
    if (isLogged) {
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
        <Head>
          <meta
            name="description"
            content="The hidden place where knowledge is shared."
          />
          <link
            href="https://fonts.googleapis.com/css?family=Lato:400,700&display=swap"
            rel="stylesheet"
          />
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <GlobalStyle />
        <NavBar />
        <ComponentWrapper>
          <Component {...pageProps} />
        </ComponentWrapper>
      </UserContext.Provider>
    );
  }
}

const ComponentWrapper = styled.div`
  padding: 60px 15px;
`;

export default Website;

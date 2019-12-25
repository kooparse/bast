import React from "react";
import Head from "next/head";
import App from "next/app";
import styled from "styled-components";
import NavBar from "../components/NavBar";
import { createGlobalStyle } from "styled-components";
import { Grommet, Menu, Header, Button, Box, Anchor, Main } from "grommet";
import { grommet, dark } from "grommet/themes";
import { Home } from "grommet-icons";
import api, { setAuthorization, isLogged } from "../utils/api";
import { UserContext } from "../utils/context";

type User = {
  username: string;
  id: number;
};

interface IState {
  user: User;
}

const theme = {
  global: {
    font: {
      family: "Helvetica",
      size: "14px",
      height: "20px"
    }
  }
};

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
        <Grommet theme={theme}>
          <NavBar />
          <Main pad="small">
            <Component {...pageProps} />
          </Main>
        </Grommet>
      </UserContext.Provider>
    );
  }
}

export default Website;

import React, { Component } from "react";
import Router from "next/router";
import styled from "styled-components";
import { UserContext } from "../../utils/context";
import { Grommet, Menu, Header, Button, Box, Anchor, Heading } from "grommet";

class NavBar extends Component {
  static contextType = UserContext;

  logout = () => {
    window.localStorage.removeItem("token");
    this.context.setUser({});
    Router.push("/");
  };

  render() {
    const { user } = this.context;
    const isConnected = !!user.id;

    return (
      <Header
        as="header"
        margin="none"
        background="brand"
        pad="medium"
        height="xxsmall"
      >
        <Anchor size="medium" label="Home" href="/" color="white" />
        {!isConnected ? (
          <Box direction="row" gap="medium">
            <Anchor label="Login" href="/login" color="white" />
            <Anchor label="Register" href="/register" color="white" />
          </Box>
        ) : (
          <Menu
            label={user.email}
            items={[{ label: "logout", onClick: this.logout }]}
          />
        )}
      </Header>
    );
  }
}

export default NavBar;

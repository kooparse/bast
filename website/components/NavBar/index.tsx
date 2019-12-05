import React, { Component } from "react";
import Router from "next/router";
import styled from "styled-components";
import Link from "../Link";
import Separator from "../Separator";
import Unlogged from "./Unlogged";
import Logged from "./Logged";
import { UserContext } from "../../utils/context";

class NavBar extends Component {
  static contextType = UserContext;

  logout = () => {
    window.localStorage.removeItem("token");
    this.context.setUser({});
    Router.push("/");
  };

  render() {
    const { user } = this.context;

    return (
      <NavBarWrapper>
        <Inner>
          <div>
            <Link nude href="/" text="Home" />
            {!!user.id && (
              <>
                <Separator />
                <Link nude href="/new_website" text="Create website" />
              </>
            )}
          </div>
          <div />
          {!!user.id ? (
            <Logged logout={this.logout} username={user.email} />
          ) : (
            <Unlogged />
          )}
        </Inner>
      </NavBarWrapper>
    );
  }
}

const NavBarWrapper = styled.nav`
  z-index: 99;
  position: fixed;
  width: 100%;
  background-color: #111;
  color: #ddd;
  font-size: 16px;
  border-bottom: 1px solid #333;
  height: 50px;
  top: 0;
  right: 0;
`;

const Inner = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  padding: 0px 20px;
`;

export default NavBar;

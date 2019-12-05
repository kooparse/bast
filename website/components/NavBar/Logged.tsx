import React from "react";
import styled from "styled-components";
import Separator from "../Separator";
import Button from "../Button";

type Props = {
  username: string;
  logout: () => void;
};

const Logged = ({ username, logout }: Props) => (
  <LoggedStyles>
    <NavTab>
      {username}
      <Separator />
    </NavTab>
    <NavTab>
      <Button onClick={logout} text="logout" />
    </NavTab>
  </LoggedStyles>
);

const LoggedStyles = styled.div`
  display: flex;
  align-items: center;
`;

const NavTab = styled.div`
  padding: 0px 5.5px;
`;

export default Logged;

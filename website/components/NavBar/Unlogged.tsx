import React from "react";
import styled from "styled-components";
import Link from "../Link";

const Unlogged = () => (
  <UnloggedStyles>
    <Link href="/login" text="Login" />
    <Link href="/register" text="Register" />
  </UnloggedStyles>
);

const UnloggedStyles = styled.div`
  display: flex;
  align-items: center;
`;

export default Unlogged;

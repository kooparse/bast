import PropTypes from "prop-types";
import React from "react";
import NextLink from "next/link";
import styled from "styled-components";
import { ButtonStyles } from "../Button";

const Link = ({ nude = false, text, ...props }) => (
  <NextLink href={props.href} {...props}>
    {nude ? (
      <NudeStyles>{text}</NudeStyles>
    ) : (
      <LinkStyles as="a">{text}</LinkStyles>
    )}
  </NextLink>
);

Link.propTypes = {
  nude: false
};

Link.propTypes = {
  text: PropTypes.string.isRequired,
  nude: PropTypes.bool
};

const LinkStyles = styled(ButtonStyles)`
  margin: 0px 5.5px;
`;

const NudeStyles = styled.a`
  padding: 0px 5.5px;
  color: currentColor;
  cursor: pointer;
  transition: color 0.5s ease;

  :hover {
    color: #fff;
  }
`;

export default Link;

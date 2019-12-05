import PropTypes from "prop-types";
import React from "react";
import styled from "styled-components";

type Props = {
  onClick?: () => any;
  text: string;
  type?: string;
  disabled?: boolean;
};

const Button = ({ onClick, text, ...props }: Props) => {
  return (
    <ButtonStyles {...props} onClick={onClick}>
      {text}
    </ButtonStyles>
  );
};

export const ButtonStyles = styled.button`
  font-size: 16px;
  margin-top: ${props => (props.type === "submit" ? "12px" : "0")};
  padding: 5.5px 15px;
  border-style: none;
  border-radius: 3px;
  color: #ddd;
  background-color: #ff2e51;
  cursor: pointer;
  transition: opacity 0.5s ease, color 0.3s ease;
  opacity: ${props => (props.disabled ? ".5" : "1")};

  :hover {
    opacity: 0.8;
    color: #fff;
  }
`;

export default Button;

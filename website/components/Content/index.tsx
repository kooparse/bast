import PropTypes from 'prop-types';
import React from 'react';
import styled from 'styled-components';

const Content = ({ title, children }) => (
  <Wrapper>
    {!!title && <Title>{title}</Title>}
    {children}
  </Wrapper>
);

Content.defaultProps = {
  title: ''
};

Content.propTypes = {
  children: PropTypes.object.isRequired,
  title: PropTypes.string
};

const Title = styled.div`
  color: #9f9f9f;
  padding: 0px 0px 10px;
  font-size: 36px;
  border-bottom: 1px solid currentColor;
  margin-bottom: 30px;
`;

const Wrapper = styled.div`
  margin: auto;
  padding: 20px;
  max-width: 800px;
  border-radius: 3px;
  background: #111;
`;

export default Content;

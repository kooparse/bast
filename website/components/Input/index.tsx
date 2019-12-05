import PropTypes from 'prop-types';
import React from 'react';
import styled from 'styled-components';

const InputField = ({
  label,
  type,
  name,
  id,
  placeholder,
  onChange,
  onBlur,
  value,
  error,
  ...props
}) => (
  <FieldStyles>
    {!!label && (
      <Label>
        <label htmlFor={id}>{label}</label>
      </Label>
    )}
    <InputStyles
      type={type}
      name={name}
      placeholder={placeholder}
      id={id}
      onChange={onChange}
      onBlur={onBlur}
      value={value}
      hasError={!!error}
      {...props}
    />
    <ErrorStyles>{error}</ErrorStyles>
  </FieldStyles>
);

const ErrorStyles = styled.div`
  font-size: 12px;
  position: absolute;
  right: 0px;
  color: red;
  margin-top: 2.5px;
`;

const Label = styled.div`
  font-size: 18px;
  color: #9f9f9f;
  margin-bottom: 5px;
`;

const FieldStyles = styled.div`
  position: relative;
  margin: 12px 0px;
  color: #ddd;
`;

const InputStyles = styled.input`
  width: 100%;
  box-sizing: border-box;
  font-size: 16px;
  padding: 5px 10px;
  border-radius: 3px;
  border: 2px solid;
  border-color: ${props =>
    props.hasError ? '#ff000061' : 'rgba(255, 255, 255, 0.1)'};
  color: #ddd;
  background: #111;
`;

InputField.defaultProps = {
  placeholder: '',
  label: ''
};

InputField.propTypes = {
  label: PropTypes.string,
  type: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  id: PropTypes.string.isRequired,
  onChange: PropTypes.func.isRequired,
  onBlur: PropTypes.func.isRequired,
  placeholder: PropTypes.string,
  value: PropTypes.string,
  error: PropTypes.string
};

export default InputField;

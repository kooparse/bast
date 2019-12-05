/*eslint no-console: ["error", { allow: ["error"] }] */
import React, { Component } from "react";
import { Formik } from "formik";
import Router from "next/router";
import styled from "styled-components";
import InputField from "../components/Input";
import Button from "../components/Button";
import Content from "../components/Content";

const initialValues = {
  email: "",
  username: "",
  password: ""
};

class Register extends Component {
  // Register new user then login with it.
  onSubmit = form => {
    // Signup with form data.
    Router.push("/");
  };

  render() {
    return (
      <Content title="Create a new account">
        <Formik initialValues={initialValues} onSubmit={this.onSubmit}>
          {({
            values,
            touched,
            errors,
            handleChange,
            handleBlur,
            isSubmitting,
            handleSubmit
          }) => (
            <form onSubmit={handleSubmit}>
              <Wrapper>
                <InputField
                  label="Email"
                  type="email"
                  name="email"
                  id="email"
                  placeholder="Your email address"
                  onChange={handleChange}
                  onBlur={handleBlur}
                  value={values.email}
                  error={errors.email && touched.email && errors.email}
                />
                <InputField
                  label="Password"
                  type="password"
                  name="password"
                  placeholder="Your secret password"
                  id="password"
                  onChange={handleChange}
                  onBlur={handleBlur}
                  value={values.password}
                  error={errors.password && touched.password && errors.password}
                />
                <Button type="submit" disabled={isSubmitting} text="Submit" />
              </Wrapper>
            </form>
          )}
        </Formik>
      </Content>
    );
  }
}

const Wrapper = styled.div`
  width: 40%;
`;

export default Register;

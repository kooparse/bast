import React, { Component } from "react";
import { Formik } from "formik";
import Router from "next/router";
import styled from "styled-components";
import InputField from "../components/Input";
import Button from "../components/Button";
import Content from "../components/Content";
import api, { setToken } from "../utils/api";

const initialValues = {
  domain: ""
};

class CreateWebsite extends Component {
  onSubmit = async form => {
    try {
      await api.post("/website", form);
      Router.push("/");
    } catch (e) {
      console.error(e);
    }
  };

  render() {
    return (
      <Content title="Create new website">
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
                  label="Domain"
                  type="text"
                  name="domain"
                  id="domain"
                  placeholder="domain address"
                  onChange={handleChange}
                  onBlur={handleBlur}
                  value={values.domain}
                  error={errors.domain && touched.domain && errors.domain}
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

export default CreateWebsite;

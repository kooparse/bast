import React, { Component } from "react";
import { Formik } from "formik";
import Router from "next/router";
import styled from "styled-components";
import { Form, FormField, TextInput, Box, Button } from "grommet";
import api, { setToken } from "../utils/api";
import FormLayout from "../components/FormLayout";

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
      <FormLayout>
        <Form onSubmit={this.onSubmit}>
          <FormField
            label="Domain Address"
            name="domain"
            type="domain"
            placeholder="google.com"
            required
          />
          <Box direction="row" margin={{ top: "medium" }}>
            <Button type="submit" label="Submit" primary />
          </Box>
        </Form>
      </FormLayout>
    );
  }
}

const Wrapper = styled.div`
  width: 40%;
`;

export default CreateWebsite;

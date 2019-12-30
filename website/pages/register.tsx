import React, { Component } from "react";
import Router from "next/router";
import { Form, FormField, TextInput, Box, Button } from "grommet";
import api, { setToken } from "../utils/api";
import { UserContext } from "../utils/context";
import FormLayout from "../components/FormLayout";

class Register extends Component {
  static contextType = UserContext;
  // Register new user then login with it.
  onSubmit = async form => {
    try {
      // First register new user.
      await api.post("/register", form);
      // Then logged new user.
      const { data } = await api.post("/login", form);
      // Store all info in localStorage + set axios header.
      setToken(data.token);
      this.context.setUser(data.user);
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
            label="Email"
            name="email"
            type="email"
            placeholder="test@protonmail.com"
            required
          />
          <FormField
            label="Password"
            name="password"
            type="password"
            placeholder="Your secret password"
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

export default Register;

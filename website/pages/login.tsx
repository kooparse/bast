import React, { Component } from "react";
import Router from "next/router";
import Content from "../components/Content";
import FormLayout from "../components/FormLayout";
import { Form, FormField, TextInput, Box, Button } from "grommet";
import api, { setToken } from "../utils/api";
import { UserContext } from "../utils/context";

class Login extends Component {
  static contextType = UserContext;

  onSubmit = async (form) => {
    try {
      const { data } = await api.post("/login", form.value);
      console.log(data);
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

export default Login;

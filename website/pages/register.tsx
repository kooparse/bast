import React, { Component } from "react";
import Router from "next/router";
import { Formik, Form, Field } from "formik";
import {
  FormControl,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Input,
  Button
} from "@chakra-ui/core";
import api, { setToken } from "../utils/api";
import { UserContext } from "../utils/context";
import FormLayout from "../components/FormLayout";

class Register extends Component {
  static contextType = UserContext;

  render() {
    return (
      <FormLayout>
        <Formik
          initialValues={{ email: "", password: "" }}
          onSubmit={async (values, actions) => {
            try {
              // First register new user.
              await api.post("/register", values);
              // Then logged new user.
              const { data } = await api.post("/login", values);
              // Store all info in localStorage + set axios header.
              setToken(data.token);
              this.context.setUser(data.user);
              Router.push("/");
            } catch (e) {
              console.error(e);

              // Cleanup.
              actions.setSubmitting(false);
            }
          }}
        >
          {props => (
            <form onSubmit={props.handleSubmit}>
              <Field name="email">
                {({ field, form }) => (
                  <FormControl
                    p={4}
                    isRequired
                    isInvalid={form.errors.email && form.touched.email}
                  >
                    <FormLabel htmlFor="email">Email address</FormLabel>
                    <Input
                      type="email"
                      id="email"
                      aria-describedby="email-helper-text"
                      {...field}
                    />
                    <FormHelperText id="email-helper-text">
                      We'll never share your email.
                    </FormHelperText>
                  </FormControl>
                )}
              </Field>

              <Field name="password">
                {({ field, form }) => (
                  <FormControl
                    isRequired
                    p={4}
                    isInvalid={form.errors.password && form.touched.password}
                  >
                    <FormLabel htmlFor="password">
                      Your secret password
                    </FormLabel>
                    <Input
                      type="password"
                      id="password"
                      aria-describedby="password-helper-text"
                      {...field}
                    />
                  </FormControl>
                )}
              </Field>

              <Button
                ml={4}
                mt={4}
                variantColor="teal"
                isLoading={props.isSubmitting}
                type="submit"
              >
                Submit
              </Button>
            </form>
          )}
        </Formik>
      </FormLayout>
    );
  }
}

export default Register;

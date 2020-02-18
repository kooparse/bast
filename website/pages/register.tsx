import React, { Component, useContext } from "react";
import Router from "next/router";
import { useFormik } from "formik";
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

export default () => {
  const ctx = useContext(UserContext);
  const formik = useFormik({
    initialValues: { email: "", password: "" },
    onSubmit: async (values, actions) => {
      try {
        // First register new user.
        await api.post("/register", values);
        // Then logged new user.
        const { data } = await api.post("/login", values);
        // Store all info in localStorage + set axios header.
        setToken(data.token);
        ctx.setUser(data.user);
        Router.push("/");
      } catch (e) {
        console.error(e);

        // Cleanup.
        actions.setSubmitting(false);
      }
    }
  });

  return (
    <FormLayout>
      <form onSubmit={formik.handleSubmit}>
        <FormControl
          p={4}
          isRequired
          isInvalid={formik.errors.email && formik.touched.email}
        >
          <FormLabel htmlFor="email">Email address</FormLabel>
          <Input
            type="email"
            id="email"
            aria-describedby="email-helper-text"
            value={formik.values.email}
            onChange={formik.handleChange}
          />
          <FormHelperText id="email-helper-text">
            We'll never share your email.
          </FormHelperText>
        </FormControl>

        <FormControl
          isRequired
          p={4}
          isInvalid={formik.errors.password && formik.touched.password}
        >
          <FormLabel htmlFor="password">Your secret password</FormLabel>
          <Input
            type="password"
            id="password"
            name="password"
            aria-describedby="password-helper-text"
            value={formik.values.password}
            onChange={formik.handleChange}
          />
        </FormControl>

        <Button
          ml={4}
          mt={4}
          variantColor="teal"
          isLoading={formik.isSubmitting}
          type="submit"
        >
          Submit
        </Button>
      </form>
    </FormLayout>
  );
};

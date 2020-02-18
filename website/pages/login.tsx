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
        const { data } = await api.post("/login", values);
        console.log(data);

        setToken(data.token);
        ctx.setUser(data.user);
        Router.push("/");
      } catch (e) {
        console.error(e);
        actions.setSubmitting(false);
      }
    }
  });

  return (
    <FormLayout>
      <form onSubmit={formik.handleSubmit}>
        <FormControl p={4}>
          <FormLabel htmlFor="email">Email address</FormLabel>
          <Input
            type="email"
            id="email"
            aria-describedby="email-helper-text"
            value={formik.values.email}
            onChange={formik.handleChange}
          />
        </FormControl>

        <FormControl p={4}>
          <FormLabel htmlFor="password">Your password</FormLabel>
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
          Login
        </Button>
      </form>
    </FormLayout>
  );
};

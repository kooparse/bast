import React, { useContext, ReactElement } from "react";
import Router from "next/router";
import { useFormik } from "formik";
import {
  useToast,
  FormControl,
  FormLabel,
  Input,
  Button
} from "@chakra-ui/core";
import api, { setToken, setAuthorization } from "../utils/api";
import { UserContext } from "../utils/context";
import { errorLogin } from "../utils/messages";
import FormLayout from "../components/FormLayout";

const initialValues = { email: "", password: "" };

const Login: React.FC = (): ReactElement => {
  const ctx = useContext(UserContext);
  const toast = useToast();

  const formik = useFormik({
    initialValues,
    onSubmit: async (values, actions) => {
      try {
        const { data } = await api.post("/login", values);

        setToken(data.token);
        setAuthorization();

        ctx.setUser(data.user);
        Router.push("/");
      } catch (e) {
        toast(errorLogin);
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
            isRequired
            type="email"
            id="email"
            placeholder="email@yours.com"
            aria-describedby="email-helper-text"
            value={formik.values.email}
            onChange={formik.handleChange}
          />
        </FormControl>

        <FormControl p={4}>
          <FormLabel htmlFor="password">Password</FormLabel>
          <Input
            isRequired
            type="password"
            id="password"
            placeholder="Secret password..."
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

export default Login;

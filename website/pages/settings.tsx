import React, { ReactElement, useContext, useState, useEffect } from "react";
import { useFormik } from "formik";
import Router from "next/router";
import {
  useToast,
  useColorMode,
  Heading,
  Box,
  Flex,
  Button,
  Skeleton,
  Alert,
  AlertTitle,
  Text,
  Select,
  Code,
  FormControl,
  FormLabel,
  Input,
} from "@chakra-ui/react";
import CodeSnippet from "../components/CodeSnippet";
import { UserContext } from "../utils/context";
import {
  errorDeleteWebsite,
  successDeleteWebsite,
  errorCreateWebsite,
  errorFetchWebsites,
  successCreateWebsite,
} from "../utils/messages";
import api, { isLogged } from "../utils/api";

const Settings: React.FC = (): ReactElement => {
  const toast = useToast();
  const { colorMode } = useColorMode();
  const { user, loading: userIsLoading } = useContext(UserContext);

  const [loading, setLoading] = useState(true);
  const [websites, setWebsites] = useState([]);
  // Not very "clean".
  const [isDeletedChanged, setIsDeletedChanged] = useState(false);
  const [selectedWebsite, setSelected] = useState(null);
  const current = websites.find((w) => `${w.id}` === `${selectedWebsite}`);

  const handleDelete = async (): Promise<void> => {
    try {
      if (confirm("Are you really sure?")) {
        await api.delete(`websites/${selectedWebsite}`);
        setIsDeletedChanged(!isDeletedChanged);
        toast(successDeleteWebsite);
      }
    } catch (err) {
      toast(errorDeleteWebsite);
      console.error(err);
    }
  };

  useEffect(() => {
    const fetch = async (): Promise<void> => {
      try {
        const { data } = await api.get("/websites");
        if (data.length) {
          setWebsites(data);
          setSelected(data[0].id);
        }
      } catch (err) {
        console.error(err);
        toast(errorFetchWebsites);
      }
      setLoading(false);
    };

    if (isLogged()) {
      setLoading(true);
      fetch();
    } else {
      Router.push("/login");
    }
  }, [isDeletedChanged]);

  const formik = useFormik({
    initialValues: { domain: "" },
    validate: (values) => {
      const errors: { domain?: string } = {};

      if (
        !/^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}$/i.test(
          values.domain
        )
      ) {
        errors.domain = "Wrong domain name";
      }

      return errors;
    },

    onSubmit: async (values, actions) => {
      try {
        const { data } = await api.post("/websites", values);
        setWebsites([data, ...websites]);
        setSelected(data.id);
        actions.resetForm();
        toast(successCreateWebsite);
      } catch (err) {
        console.error(err);
        toast(errorCreateWebsite);
        actions.setSubmitting(false);
      }
    },
  });

  const hasError =
    formik.errors.domain && formik.touched.domain && !!formik.submitCount;

  const bg = { light: "gray.50", dark: "gray.900" };
  const color = { light: "grey.900", dark: "gray.50" };

  const SelectContent = websites.length ? (
    <>
      <Flex justifyContent="space-between">
        <Select
          width="300px"
          value={current?.id}
          onChange={(event): void => setSelected(event.target.value)}
        >
          {websites.map((w, i) => (
            <option key={i} style={{ color: "initial" }} value={w.id}>
              {w.domain}
            </option>
          ))}
        </Select>
        <Button display={{ sm: "none", md: "initial" }} onClick={handleDelete}>
          Delete &#128128;
        </Button>
      </Flex>
      {current && !!user?.id && (
        <Box my="2">
          <CodeSnippet website={current} />
        </Box>
      )}
    </>
  ) : (
    <Alert
      variant="subtle"
      flexDirection="column"
      justifyContent="center"
      textAlign="center"
      borderRadius="md"
      bg={bg[colorMode]}
      color={color[colorMode]}
    >
      <AlertTitle fontSize="md">
        You don&apos;t have any website yet!
      </AlertTitle>
    </Alert>
  );

  return (
    <Box>
      <Box mb="20">
        <Heading size="lg" pb="5">
          Options & Parameters
        </Heading>
        <Text>
          This snippet should be added near the top of the{" "}
          <Code>{"<head>"}</Code> tag and before any other script or css tags.
          <br />
          This code create a <Code>{"<script>"}</Code> element that starts
          asynchronously downloading the external analytic file. It will falled
          back on <Code>defer</Code> methods if browser does not support{" "}
          <Code>async</Code>. However, if your target audience are using mostly
          very old browsers, you should put this script at the bottom of your{" "}
          <Code>{"<body>"}</Code>.
        </Text>
        <Box my="8">
          {!loading && !userIsLoading ? (
            SelectContent
          ) : (
            <>
              <Skeleton h="60px" width="100%" mb="3" />
              <Skeleton h="200px" width="100%" />
            </>
          )}
        </Box>
      </Box>

      <Box my="8">
        <Heading size="lg">New website</Heading>
        <form onSubmit={formik.handleSubmit}>
          <FormControl isRequired py={4}>
            <FormLabel htmlFor="domain">Domain name</FormLabel>
            <Input
              isRequired
              isInvalid={hasError}
              errorBorderColor="red.300"
              type="domain"
              id="domain"
              placeholder="mydomain.com"
              aria-describedby="domain-helper-text"
              value={formik.values.domain}
              onChange={formik.handleChange}
            />
            {hasError && (
              <Text fontSize="sm" color="tomato" mt={1}>
                {formik.errors.domain} &#128577;
              </Text>
            )}
          </FormControl>

          <Button
            mt={2}
            colorScheme="teal"
            isLoading={formik.isSubmitting}
            type="submit"
          >
            Submit
          </Button>
        </form>
      </Box>
    </Box>
  );
};

export default Settings;

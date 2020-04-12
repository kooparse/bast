import React, { ReactElement, useContext, useState, useEffect } from "react";
import { useFormik } from "formik";
import Router from "next/router";
import {
  useToast,
  useColorMode,
  Heading,
  Box,
  Skeleton,
  Alert,
  AlertTitle,
  Text,
  Select,
  Code,
  FormControl,
  FormLabel,
  Input,
  Button
} from "@chakra-ui/core";
import CodeSnippet from "../components/CodeSnippet";
import { UserContext } from "../utils/context";
import {
  errorCreateWebsite,
  errorFetchWebsites,
  successCreateWebsite
} from "../utils/messages";
import api, { isLogged } from "../utils/api";

const Settings: React.FC = (): ReactElement => {
  const toast = useToast();
  const { colorMode } = useColorMode();
  const { user, loading: userIsLoading } = useContext(UserContext);

  const [loading, setLoading] = useState(true);
  const [websites, setWebsites] = useState([]);
  const [selectedWebsite, setSelected] = useState(null);
  const current = websites.find(w => `${w.id}` === `${selectedWebsite}`);

  useEffect(() => {
    const fetch = async (): Promise<void> => {
      try {
        const { data } = await api.get("/websites");
        if (data.length) {
          setWebsites(data);
          setSelected(data[0].id);
        }
      } catch (err) {
        toast(errorFetchWebsites);
        console.error(err);
      }
      setLoading(false);
    };

    if (isLogged()) {
      setLoading(true);
      fetch();
    } else {
      Router.push("/login");
    }
  }, []);

  const formik = useFormik({
    initialValues: { domain: "" },
    onSubmit: async (values, actions) => {
      try {
        const { data } = await api.post("/websites", values);
        setWebsites([data, ...websites]);
        setSelected(data.id);
        toast(successCreateWebsite);
      } catch (e) {
        toast(errorCreateWebsite);
        actions.setSubmitting(false);
      }
    }
  });

  const bg = { light: "gray.50", dark: "gray.900" };
  const color = { light: "grey.900", dark: "gray.50" };

  const SelectContent = websites.length ? (
    <>
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
      {current && !!user?.id && (
        <Box my="2">
          <CodeSnippet user={user} website={current} />
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
          asynchronously downloading the analytic file.
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
              type="domain"
              id="domain"
              placeholder="mydomain.com"
              aria-describedby="domain-helper-text"
              value={formik.values.domain}
              onChange={formik.handleChange}
            />
          </FormControl>

          <Button
            mt={2}
            variantColor="teal"
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

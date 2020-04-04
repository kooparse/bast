import React, { ReactElement, useContext, useState, useEffect } from "react";
import { useFormik } from "formik";
import {
  useToast,
  Heading,
  Box,
  Text,
  Code,
  FormControl,
  FormLabel,
  Input,
  Button
} from "@chakra-ui/core";
import CodeSnippet from "../components/CodeSnippet";
import { UserContext } from "../utils/context";
import { errorCreateWebsite } from "../utils/messages";
import api, { isLogged } from "../utils/api";

const Settings: React.FC = (): ReactElement => {
  const toast = useToast();
  const [websites, setWebsites] = useState([]);
  const { user } = useContext(UserContext);

  const formik = useFormik({
    initialValues: { domain: "" },
    onSubmit: async (values, actions) => {
      try {
        const { data } = await api.post("/websites", values);
        setWebsites([data, ...websites]);
      } catch (e) {
        toast(errorCreateWebsite);
        actions.setSubmitting(false);
      }
    }
  });

  useEffect(() => {
    const fetch = async (): Promise<void> => {
      try {
        const { data } = await api.get("/websites");
        setWebsites(data);
      } catch (err) {
        console.error(err);
      }
    };

    if (isLogged()) {
      fetch();
    }
  }, []);

  return (
    <div>
      <Heading as="h1">Settings</Heading>

      <Box my="20">
        <Heading size="lg">Add new domain</Heading>
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

      <Box>
        <Text>
          The snippet should be added near the top of the{" "}
          <Code>{"<head>"}</Code> tag and before any other script or css tags.
          <br />
          This code create a <Code>{"<script>"}</Code> element that starts
          asynchronously downloading the analytic file.
        </Text>
        {websites.map((website, i) => {
          return (
            <Box py="5" mb="5" key={i}>
              <Heading size="sm">
                <Text as="span" color="teal.500">
                  {website.domain.toUpperCase()}
                </Text>{" "}
                snippet:
              </Heading>
              <CodeSnippet user={user} website={website} />
            </Box>
          );
        })}
      </Box>
    </div>
  );
};

export default Settings;

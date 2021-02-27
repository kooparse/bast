import React, { ReactElement } from "react";
import { Box, Flex } from "@chakra-ui/react";

const FormLayout = ({ children }: { children: ReactElement }): ReactElement => (
  <Flex justify="center">
    <Box w={{ sm: "100%", md: "60%" }} p={4}>
      {children}
    </Box>
  </Flex>
);

export default FormLayout;

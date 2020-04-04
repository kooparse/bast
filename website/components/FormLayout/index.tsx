import React, { ReactElement } from "react";
import { Box, Flex } from "@chakra-ui/core";

const FormLayout = ({
  children
}: {
  children: ReactElement;
}): ReactElement => (
  <Flex justify="center">
    <Box w="60%" p={4}>
      {children}
    </Box>
  </Flex>
);

export default FormLayout;

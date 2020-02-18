import { Box, Flex } from "@chakra-ui/core";

const FormLayout = ({ children }) => (
  <Flex justify="center">
    <Box w="40%" p={4}>
      {children}
    </Box>
  </Flex>
);

export default FormLayout;

import { Box } from "grommet";

const FormLayout = ({ children }) => (
  <Box fill align="center" justify="center">
    <Box width="medium">{children}</Box>
  </Box>
);

export default FormLayout;

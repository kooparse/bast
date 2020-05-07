import React, { ReactElement } from "react";
import { Box } from "@chakra-ui/core";
import Table from "../Table";

const OperatingSystemTable = ({
  systems,
  loading,
  title,
}: {
  title: string;
  systems: OS[] | Browser[];
  loading: boolean;
}): ReactElement => {
  let totalCounter = 0;

  systems.forEach(({ counter }) => (totalCounter += counter));

  return (
    <Box rounded="md" borderWidth="1px" p="5" mt="8">
      <Table
        loading={loading}
        title={title}
        rows={systems.map(({ name, counter }) => {
          const percent = Math.round((counter / totalCounter) * 100);

          return {
            label: name,
            tooltipLabel: `${percent}%`,
            percent,
          };
        })}
      />
    </Box>
  );
};

export default OperatingSystemTable;

import React, { ReactElement } from "react";
import { Box } from "@chakra-ui/react";
import Table from "../Table";

const ReferrerTable = ({
  referrers,
  loading,
}: {
  loading: boolean;
  referrers: Referrer[];
}): ReactElement => {
  let totalCounter = 0;

  referrers.forEach(({ count }) => (totalCounter += count));

  return (
    <Box rounded="md" borderWidth="1px" p="5" mt={{ sm: 0, md: 8 }}>
      <Table
        title="Referrers"
        rows={referrers.map(({ name, count }) => {
          const percent = Math.round((count / totalCounter) * 100);
          return {
            label: name,
            tooltipLabel: `${percent}% / ${count} time${
              count === 1 ? "" : "s"
            }`,
            percent,
          };
        })}
        loading={loading}
      />
    </Box>
  );
};

export default ReferrerTable;

import React, { ReactElement } from "react";
import { Box } from "@chakra-ui/core";
import Table from "../Table";

const ReferrerTable = ({
  referrers,
  loading
}: {
  loading: boolean;
  referrers: Referrer[];
}): ReactElement => {
  let maxCount = 0;
  referrers.forEach(({ count }) => (maxCount = Math.max(count, maxCount)));

  return (
    <Box rounded="md" borderWidth="1px" p="5" mt="8">
      <Table
        title="Referrers"
        rows={referrers.map(({ name, count }) => ({
          label: name,
          tooltipLabel: `Count ${count}`,
          percent: (count / maxCount) * 100
        }))}
        loading={loading}
      />
    </Box>
  );
};

export default ReferrerTable;

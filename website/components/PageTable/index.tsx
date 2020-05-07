import React, { ReactElement } from "react";
import { Box } from "@chakra-ui/core";
import Table from "../Table";

const PageTable = ({
  pages,
  loading,
}: {
  pages: Page[];
  loading: boolean;
}): ReactElement => {
  let totalPageviews = 0;
  pages.forEach(({ pageviews }) => (totalPageviews += pageviews));

  return (
    <Box rounded="md" borderWidth="1px" p="5" mt="8">
      <Table
        loading={loading}
        title="Pages"
        rows={pages.map(({ name, pageviews, users, sessions }) => ({
          label: name,
          tooltipLabel: `${pageviews} pageviews / ${users} users / ${sessions} sessions`,
          percent: Math.round((pageviews / totalPageviews) * 100),
        }))}
      />
    </Box>
  );
};

export default PageTable;

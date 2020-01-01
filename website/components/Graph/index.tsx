import React from "react";
import { format } from "date-fns";
import {
  Box,
  Heading,
  Text,
  DataTable,
  Meter,
  Select,
  TextArea,
  Chart
} from "grommet";

const Bar = ({ max, value }: { max: number; value: GraphDatum }) => (
  <Box flex={false} basis="xsmall" align="center">
    <Chart
      overflow
      bounds={[
        [0, 2],
        [0, max]
      ]}
      type="bar"
      values={[{ value: [1, value.uniques] }]}
      size={{ height: "small", width: "auto" }}
      gap="small"
    />
    <Box align="center" margin={{ vertical: "small" }}>
      <Text size="small">{value.uniques}</Text>
      <Text weight="bold">{format(value.date, "MMM yyyy")}</Text>
    </Box>
  </Box>
);

const Graph = ({ data }: { data: GraphDatum[] }) => {
  console.log(data);
  const max = Math.max(...data.map(d => d.uniques));

  return (
    <Box>
      <Box margin={{ vertical: "medium" }}>
        <Heading level={2} margin="small">
          Overall stats
        </Heading>
        <Box direction="row" margin={{ vertical: "small" }}>
          {data.map((d, i) => (
            <Bar max={max} value={d} key={i} />
          ))}
        </Box>
      </Box>
    </Box>
  );
};

export default Graph;

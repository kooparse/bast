import React, { ReactElement, useState } from "react";
import { ArrowBackIcon, ArrowForwardIcon } from "@chakra-ui/icons";
import {
  Flex,
  Heading,
  Alert,
  Text,
  Box,
  useColorMode,
  Tooltip,
  Skeleton,
  IconButton,
} from "@chakra-ui/react";

const Table = ({
  title,
  loading,
  rows,
  itemPerPage = 5,
}: {
  title: string;
  loading: boolean;
  rows: {
    label?: string;
    tooltipLabel: string | number;
    percent: number;
  }[];
  itemPerPage?: number;
}): ReactElement => {
  const { colorMode } = useColorMode();
  const bg = { light: "gray.50", dark: "gray.900" };
  const color = { light: "grey.900", dark: "gray.50" };

  const totalPageNumber = Math.ceil(rows.length / itemPerPage);
  const [currentPage, setCurrentPage] = useState(1);

  const items = rows.slice(
    (currentPage - 1) * itemPerPage,
    currentPage * itemPerPage
  );

  return loading ? (
    <Skeleton w="100%" h="100px" />
  ) : (
    <>
      <table style={{ width: "100%" }}>
        <thead>
          <tr style={{ textAlign: "left" }}>
            <th style={{ paddingBottom: 20 }}>
              <Heading as="h4" size="md">
                {title}
              </Heading>
            </th>
          </tr>
        </thead>
        <tbody>
          {rows.length ? (
            items.map(({ label, tooltipLabel, percent }, i) => (
              <tr key={i}>
                <td style={{ paddingBottom: 8 }}>
                  <Tooltip
                    hasArrow
                    label={`${tooltipLabel}`}
                    aria-label="tooltip"
                  >
                    <Flex
                      position="relative"
                      h="8"
                      bg={colorMode === "light" ? "gray.100" : "gray.900"}
                    >
                      <Box
                        display="flex"
                        alignItems="center"
                        bg="teal.500"
                        minWidth={2}
                        _hover={{ bg: "teal.600" }}
                        width={`${percent}%`}
                      ></Box>
                      <Text
                        p={2}
                        position="absolute"
                        width={300}
                        alignSelf="center"
                        isTruncated
                      >
                        {label || `${percent}%`}
                      </Text>
                    </Flex>
                  </Tooltip>
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td>
                <Alert
                  status="info"
                  bg={bg[colorMode]}
                  color={color[colorMode]}
                >
                  There is no data yet!
                </Alert>
              </td>
            </tr>
          )}
        </tbody>
      </table>
      <Flex justify="space-between" mt={5}>
        <div>
          {currentPage < totalPageNumber && (
            <IconButton
              aria-label="previous"
              onClick={(): void => {
                if (currentPage < totalPageNumber) {
                  setCurrentPage(currentPage + 1);
                }
              }}
              icon={<ArrowBackIcon />}
              colorScheme="teal"
              variant="outline"
              size="xs"
            />
          )}
        </div>
        <div>
          {currentPage > 1 && (
            <IconButton
              aria-label="next"
              onClick={(): void => {
                if (currentPage > 1) {
                  setCurrentPage(currentPage - 1);
                }
              }}
              icon={<ArrowForwardIcon />}
              colorScheme="teal"
              variant="outline"
              size="xs"
            />
          )}
        </div>
      </Flex>
    </>
  );
};

export default Table;

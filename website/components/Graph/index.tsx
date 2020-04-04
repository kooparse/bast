import React, { ReactElement } from "react";
import format from "date-fns/format";
import { Text, Flex, Box, Tooltip, PseudoBox, Skeleton } from "@chakra-ui/core";

const Graph = ({
  data,
  loading
}: {
  data: MonthStat[];
  loading: boolean;
}): ReactElement => {
  let maxUsers = 0;
  let maxSessions = 0;

  data.forEach(({ users, sessions }) => {
    maxUsers = Math.max(users, maxUsers);
    maxSessions = Math.max(sessions, maxSessions);
  });

  return (
    <Flex
      justifyContent="flex-end"
      rounded="md"
      borderWidth="1px"
      p="5"
      h="300px"
      mt="10"
      mb="10"
    >
      {loading ? (
        <>
          {new Array(12).fill(null).map((_, i) => (
            <Flex mr="5" alignItems="flex-end" key={i}>
              <Skeleton w={13} h="80%" mx={1} />
              <Skeleton w={13} h="70%" mx={1} />
            </Flex>
          ))}
        </>
      ) : (
        <>
          {data.map((datum, i) => {
            const userRatio = (datum.users / maxUsers) * 100;
            const sessionRatio = (datum.sessions / maxSessions) * 100;
            const label = format(new Date(datum.createdAt), "MMM yy");

            return (
              <Box mr="5" pb="5" key={i}>
                <Flex
                  w="100%"
                  h="100%"
                  justifyContent="center"
                  alignItems="flex-end"
                >
                  <Tooltip
                    hasArrow
                    label={`${datum.users} users`}
                    aria-label="user-count"
                  >
                    <PseudoBox
                      bg="teal.500"
                      _hover={{ bg: "teal.600" }}
                      minHeight={5}
                      h={`${userRatio}%`}
                      w="13px"
                      mr="1"
                    />
                  </Tooltip>
                  <Tooltip
                    hasArrow
                    label={`${datum.sessions} sessions`}
                    aria-label="session-count"
                  >
                    <PseudoBox
                      bg="teal.300"
                      _hover={{ bg: "teal.400" }}
                      minHeight={5}
                      h={`${sessionRatio}%`}
                      w="13px"
                      mr="1"
                    />
                  </Tooltip>
                </Flex>
                <Text as="span" fontSize="sm">
                  {label}
                </Text>
              </Box>
            );
          })}
        </>
      )}
    </Flex>
  );
};

export default Graph;

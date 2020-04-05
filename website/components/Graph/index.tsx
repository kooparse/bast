import React, { ReactElement } from "react";
import format from "date-fns/format";
import subMonths from "date-fns/subMonths";
import { Text, Flex, Box, Tooltip, PseudoBox, Skeleton } from "@chakra-ui/core";

const Graph = ({
  data,
  from,
  loading
}: {
  data: MonthStat[];
  from: Date;
  loading: boolean;
}): ReactElement => {
  let maxUsers = 0;
  let maxSessions = 0;

  data.forEach(({ users, sessions }) => {
    maxUsers = Math.max(users, maxUsers);
    maxSessions = Math.max(sessions, maxSessions);
  });

  const indexes = data.reduce((result, datum) => {
    const date = new Date(datum.createdAt);
    const userRatio = (datum.users / maxUsers) * 100;
    const sessionRatio = (datum.sessions / maxSessions) * 100;
    const label = format(new Date(datum.createdAt), "MMM yy");

    return {
      ...result,
      [`${date.getFullYear()}-${date.getMonth() + 1}`]: {
        ...datum,
        userRatio,
        sessionRatio,
        label
      }
    };
  }, {});

  return (
    <Flex
      justifyContent="center"
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
          {new Array(11)
            .fill("")
            .map((_, i) => {
              const date = subMonths(from, i);
              const key = `${date.getFullYear()}-${date.getMonth() + 1}`;

              const datum = indexes[key]
                ? indexes[key]
                : {
                    users: 0,
                    sessions: 0,
                    userRatio: 0,
                    sessionRatio: 0,
                    label: format(date, "MMM, yy")
                  };

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
                        minHeight={1}
                        h={`${datum.userRatio}%`}
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
                        minHeight={1}
                        h={`${datum.sessionRatio}%`}
                        w="13px"
                        mr="1"
                      />
                    </Tooltip>
                  </Flex>
                  <Text as="span" fontSize="sm">
                    {datum.label}
                  </Text>
                </Box>
              );
            })
            .reverse()}
        </>
      )}
    </Flex>
  );
};

export default Graph;

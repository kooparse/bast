import React, { ReactElement, useEffect, useState } from "react";
import format from "date-fns/format";
import { Text, Flex, Tooltip, PseudoBox, Skeleton } from "@chakra-ui/core";

const Graph = ({
  data,
  loading,
}: {
  data: GraphData;
  loading: boolean;
}): ReactElement => {
  const [isOnSmallDevice, setIsOnSmallDevice] = useState(false);
  let maxUsers = 0;
  let maxSessions = 0;

  Object.values(data).forEach(({ users, sessions }) => {
    maxUsers = Math.max(users, maxUsers);
    maxSessions = Math.max(sessions, maxSessions);
  });

  useEffect(() => {
    let timeoutId = null;

    const handleResize = () => {
      clearTimeout(timeoutId);

      timeoutId = setTimeout(() => {
        let width =
          window.innerWidth ||
          document.documentElement.clientWidth ||
          document.body.clientWidth;

        setIsOnSmallDevice(width <= 767);
      }, 50);
    };

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  });

  const entries = isOnSmallDevice
    ? Object.entries(data).slice(6)
    : Object.entries(data);

  return (
    <>
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
            {new Array(isOnSmallDevice ? 5 : 11).fill(null).map((_, i) => (
              <Flex mr="5" alignItems="flex-end" key={i}>
                <Skeleton w={13} h="80%" mx={1} />
                <Skeleton w={13} h="70%" mx={1} />
              </Flex>
            ))}
          </>
        ) : (
          entries.map(([date, datum], i) => {
            const [year, month] = date.split("-");
            const userRatio = (datum.users / maxUsers) * 100;
            const sessionRatio = (datum.sessions / maxSessions) * 100;
            const label = format(
              new Date(Number(year), Number(month), 0),
              "MMM, yy"
            );

            return (
              <PseudoBox
                mr={{ xsm: "2", md: "4" }}
                _last={{ mr: "0" }}
                pb="5"
                key={i}
              >
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
                      minHeight={1}
                      h={`${sessionRatio}%`}
                      w="13px"
                      mr="1"
                    />
                  </Tooltip>
                </Flex>
                <Text as="span" fontSize="sm">
                  {label}
                </Text>
              </PseudoBox>
            );
          })
        )}
      </Flex>
    </>
  );
};

export default Graph;

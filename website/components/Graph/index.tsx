import React, { ReactElement, useEffect, useState } from "react";
import format from "date-fns/format";
import { ArrowBackIcon, ArrowForwardIcon } from "@chakra-ui/icons";
import {
  Text,
  Flex,
  Stack,
  Tooltip,
  Skeleton,
  Box,
  RadioGroup,
  ButtonGroup,
  Button,
  Radio,
} from "@chakra-ui/react";

const Graph = ({
  data,
  loading,
  view,
  onChangeView,
  onChangeRange,
}: {
  data: GraphData;
  loading: boolean;
  view: string;
  onChangeView: (_: string) => void;
  onChangeRange: (_: number) => void;
}): ReactElement => {
  const isMonth = view == "month";
  const [isOnSmallDevice, setIsOnSmallDevice] = useState(false);
  let maxUsers = 0;
  let maxSessions = 0;

  Object.values(data).forEach(({ users, sessions }) => {
    maxUsers = Math.max(users, maxUsers);
    maxSessions = Math.max(sessions, maxSessions);
  });

  useEffect(() => {
    let timeoutId = null;

    const handleResize = (): void => {
      clearTimeout(timeoutId);

      timeoutId = setTimeout(() => {
        const width =
          window.innerWidth ||
          document.documentElement.clientWidth ||
          document.body.clientWidth;

        setIsOnSmallDevice(width <= 767);
      }, 50);
    };

    handleResize();
    window.addEventListener("resize", handleResize);

    return (): void => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  const entries = isOnSmallDevice
    ? Object.entries(data).slice(isMonth ? 6 : 3)
    : Object.entries(data);

  return (
    <Box rounded="md" borderWidth="1px" p="5">
      <Flex justifyContent="space-between" mb="8">
        <ButtonGroup spacing={4}>
          <Button
            onClick={(): void => onChangeRange(-1)}
            leftIcon={<ArrowBackIcon />}
            colorScheme="teal"
            variant="outline"
            size="xs"
          >
            Previous
          </Button>
          <Button
            onClick={(): void => onChangeRange(1)}
            rightIcon={<ArrowForwardIcon />}
            colorScheme="teal"
            variant="outline"
            size="xs"
          >
            Next
          </Button>
        </ButtonGroup>
        <RadioGroup
          spacing={4}
          defaultValue={view}
          onChange={(view: string): void => onChangeView(view)}
        >
          <Stack direction="row">
            <Radio value="month">Monthly</Radio>
            <Radio value="day">Daily</Radio>
          </Stack>
        </RadioGroup>
      </Flex>

      <Flex justifyContent="center" height="300px">
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
            const width = isMonth ? 13 : 30;
            const [year, month] = date.split("-");

            const userRatio = (datum.users / maxUsers) * 100;
            const sessionRatio = (datum.sessions / maxSessions) * 100;
            const label = isMonth
              ? format(new Date(Number(year), Number(month), 0), "MMM, yy")
              : format(new Date(date), "d MMM yy");

            return (
              <Box mr={{ sm: "2", md: "4" }} _last={{ mr: "0" }} pb="5" key={i}>
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
                    <Box
                      bg="teal.500"
                      _hover={{ bg: "teal.600" }}
                      minHeight={1}
                      h={`${userRatio}%`}
                      w={width}
                      mr="1"
                    />
                  </Tooltip>
                  <Tooltip
                    hasArrow
                    label={`${datum.sessions} sessions`}
                    aria-label="session-count"
                  >
                    <Box
                      bg="teal.300"
                      _hover={{ bg: "teal.400" }}
                      minHeight={1}
                      h={`${sessionRatio}%`}
                      w={width}
                      mr="1"
                    />
                  </Tooltip>
                </Flex>
                <Text as="span" fontSize="sm">
                  {label}
                </Text>
              </Box>
            );
          })
        )}
      </Flex>
    </Box>
  );
};

export default Graph;

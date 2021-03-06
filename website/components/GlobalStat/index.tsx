import React, { ReactElement } from "react";
import {
  Flex,
  Skeleton,
  Stat,
  StatLabel,
  StatHelpText,
  StatNumber,
  StatGroup,
} from "@chakra-ui/react";

const toReadableTimeFormat = (secondes: number): string => {
  return secondes >= 60
    ? `${Math.floor(secondes / 60)}m`
    : `${Math.floor(secondes)}s`;
};

const GlobalStat = ({
  website,
  loading,
}: {
  website: Website;
  loading: boolean;
}): ReactElement => {
  return (
    <StatGroup rounded="md" borderWidth="1px" p="5" mt="10">
      <Skeleton isLoaded={!loading} width="100%">
        <Flex textAlign="center">
          <StatBox label="Pageviews" value={website.pageviews} />
          <StatBox label="Users" value={website.users} />
          <StatBox label="Sessions" value={website.sessions} />
          <StatBox
            couldHide
            label="Average Time"
            value={toReadableTimeFormat(website.avgTime)}
          />
        </Flex>
      </Skeleton>
    </StatGroup>
  );
};

const StatBox = ({
  label,
  value,
  helper,
  couldHide,
}: {
  label: string;
  value: number | string;
  helper?: string;
  couldHide?: boolean;
}): ReactElement => (
  <Stat display={{ sm: couldHide ? "none" : "initial", md: "initial" }}>
    <StatLabel>{label}</StatLabel>
    <StatNumber fontSize={{ sm: "xl", md: "3xl" }}>{value}</StatNumber>
    {!!helper && <StatHelpText>{helper}</StatHelpText>}
  </Stat>
);

export default GlobalStat;

import React, { useState, useEffect, ReactElement, useContext } from "react";
import getUnixTime from "date-fns/getUnixTime";
import endOfMonth from "date-fns/endOfMonth";
import startOfMonth from "date-fns/startOfMonth";
import subDays from "date-fns/subDays";
import subMonths from "date-fns/subMonths";
import addDays from "date-fns/addDays";
import startOfDay from "date-fns/startOfDay";
import endOfDay from "date-fns/endOfDay";
import addMonths from "date-fns/addMonths";
import Link from "next/link";
import { useRouter } from "next/router";
import {
  useToast,
  useColorMode,
  Alert,
  AlertDescription,
  AlertTitle,
  Flex,
  Heading,
  SimpleGrid,
  Select,
  Box,
} from "@chakra-ui/react";
import Graph from "../components/Graph";
import ReferrerTable from "../components/ReferrerTable";
import PageTable from "../components/PageTable";
import System from "../components/SystemTable";
import GlobalStat from "../components/GlobalStat";
import api, { isLogged } from "../utils/api";
import { errorFetchStats, errorFetchWebsites } from "../utils/messages";
import { UserContext } from "../utils/context";

const defaultStats: Stats = {
  pages: [],
  referrers: [],
  stats: {},
  systems: {
    operatingSystems: [],
    browsers: [],
    categories: [],
  },
  website: {
    domain: "",
    pageviews: 0,
    users: 0,
    sessions: 0,
    avgTime: 0,
    id: null,
  },
};

type Range = {
  start: Date;
  end: Date;
};

function computeRange(from: Date, view: string, direction: number): Range {
  const isMonth = view === "month";

  const range = {
    start: isMonth
      ? startOfMonth(subMonths(from, 10))
      : startOfDay(subDays(from, 6)),
    end: isMonth ? endOfMonth(from) : endOfDay(from),
  };

  if (direction === 1) {
    range.end = isMonth
      ? endOfMonth(addMonths(from, 10))
      : endOfDay(addDays(from, 6));
    range.start = isMonth ? startOfMonth(from) : startOfDay(from);
  }

  return range;
}

const Home: React.FC = (): ReactElement => {
  const { user, loading: userIsLoading } = useContext(UserContext);
  const router = useRouter();
  const toast = useToast();
  const { colorMode } = useColorMode();
  const bg = { light: "gray.50", dark: "gray.900" };
  const color = { light: "grey.900", dark: "gray.50" };

  const [loading, setLoading] = useState(userIsLoading);
  const [statLoading, setStatLoading] = useState(true);
  const [websites, setWebsites] = useState([]);
  const [selectedWebsiteId, setSelected] = useState("");
  const [stats, setStats] = useState(defaultStats);
  const [view, setView] = useState("month");
  const [range, changeFrom] = useState(
    computeRange(endOfMonth(new Date()), view, -1)
  );

  const { website } = stats;

  // Effect used to fetch stats from a selected domain.
  // Triggered on mount cycle and when user changes domain from
  // the <Select /> component.
  useEffect(() => {
    const fetchStat = async (): Promise<void> => {
      try {
        const { start, end } = range;

        const { data } = await api.get(
          `/stats?website_id=${selectedWebsiteId}&start=${getUnixTime(
            start
          )}&end=${getUnixTime(end)}&by=${view}`
        );
        setStats(data);
      } catch (err) {
        console.error(err);
        toast(errorFetchStats);
      }

      setStatLoading(false);
    };

    if (selectedWebsiteId) {
      setStatLoading(true);
      fetchStat();
    }
  }, [selectedWebsiteId, range, view]);

  // Effect used as ComponentDidMount, retrieves websites from the current user.
  useEffect(() => {
    const call = async (): Promise<void> => {
      const urlParams = new URLSearchParams(window.location.search);
      const queryId = urlParams.get("id") || null;

      try {
        const { data = [] } = await api.get("/websites");
        if (!data.length) {
          setLoading(false);
          return;
        }
        // Set the list of websites (used in the <Select /> component.
        setWebsites(data);
        // If we don't get domain id from the query, we select
        // and get the first one.
        if (!queryId) {
          setSelected(data[0].id);
          setLoading(false);
          return;
        }
        // If not, we find the corresponding domain and select it.
        const selected = data.find(({ id }) => id.toString() === queryId);
        setSelected(selected.id);
      } catch (err) {
        console.error(err);
        toast(errorFetchWebsites);
      }

      setLoading(false);
    };

    if (isLogged()) {
      setLoading(true);
      call();
    }
  }, []);

  if (!user && !userIsLoading) {
    return (
      <Alert
        variant="subtle"
        flexDirection="column"
        justifyContent="center"
        textAlign="center"
        height="200px"
        borderRadius="md"
        bg={bg[colorMode]}
        color={color[colorMode]}
      >
        <AlertTitle mt={4} mb={1} fontSize="lg">
          There is no landing page yet :)
          <br />
        </AlertTitle>
        <AlertDescription maxWidth="sm">
          Go to&nbsp;
          <Link href="/login">
            <a>
              <u>login</u>
            </a>
          </Link>{" "}
          page or create a&nbsp;
          <Link href="/register">
            <a>
              <u>new account</u>
            </a>
          </Link>
          .
        </AlertDescription>
      </Alert>
    );
  }

  if (!websites.length && !loading) {
    return (
      <Alert
        variant="subtle"
        flexDirection="column"
        justifyContent="center"
        textAlign="center"
        height="200px"
        borderRadius="md"
        bg={bg[colorMode]}
        color={color[colorMode]}
      >
        <AlertTitle mt={4} mb={1} fontSize="lg">
          You don&apos;t have any website yet!
        </AlertTitle>
        <AlertDescription maxWidth="sm">
          Go to your&nbsp;
          <Link href="/settings">
            <a>
              <u>settings</u>
            </a>
          </Link>{" "}
          in order to have new domains associated with your account! :)
        </AlertDescription>
      </Alert>
    );
  }

  return (
    <>
      <Flex
        flexDirection={{ sm: "column", md: "row" }}
        justifyContent="space-between"
        alignContent="center"
      >
        <Heading as="h1" mb={{ sm: 5, md: 0 }}>
          Dashboard
        </Heading>

        {!!websites.length && (
          <Select
            width={{ sm: "100%", md: 300 }}
            value={websites.find((w) => w.id === selectedWebsiteId)?.id}
            onChange={(event): void => {
              const { value: id } = event.target;
              router.replace({ pathname: "/", query: { id } });
              setSelected(id);
            }}
          >
            {websites.map((w, i) => (
              <option key={i} style={{ color: "initial" }} value={w.id}>
                {w.domain}
              </option>
            ))}
          </Select>
        )}
      </Flex>

      <GlobalStat website={website} loading={loading} />

      <Box my={10}>
        <Graph
          data={stats.stats}
          loading={statLoading}
          view={view}
          onChangeRange={(direction: number): void => {
            const range = Object.keys(stats.stats);

            direction === -1
              ? changeFrom(computeRange(new Date(range.shift()), view, -1))
              : changeFrom(computeRange(new Date(range.pop()), view, 1));
          }}
          onChangeView={(v: string): void => {
            v === "month"
              ? changeFrom(computeRange(endOfMonth(new Date()), v, -1))
              : changeFrom(computeRange(endOfDay(new Date()), v, -1));

            setView(v);
          }}
        />
      </Box>

      <SimpleGrid
        alignItems="start"
        columns={{ sm: 1, md: 2 }}
        spacing={{ sm: 10, md: 5 }}
      >
        <PageTable loading={loading} pages={stats.pages} />
        <ReferrerTable loading={loading} referrers={stats.referrers} />
      </SimpleGrid>

      <SimpleGrid columns={{ sm: 1, md: 3 }} spacing={{ sm: 10, md: 5 }}>
        <System
          loading={loading}
          systems={stats.systems.operatingSystems}
          title="Operating Systems"
        />
        <System
          loading={loading}
          systems={stats.systems.browsers}
          title="Browsers"
        />
        <System
          loading={loading}
          systems={stats.systems.categories.map((c) => ({
            ...c,
            name:
              c.name === "pc"
                ? "Desktop"
                : c.name.charAt(0).toUpperCase() + c.name.slice(1),
          }))}
          title="Categories"
        />
      </SimpleGrid>
    </>
  );
};

export default Home;

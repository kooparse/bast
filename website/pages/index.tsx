import React, { useState, useEffect, ReactElement, useContext } from "react";
import getUnixTime from "date-fns/getUnixTime";
import Link from "next/link";
import { useRouter } from "next/router";
import {
  useToast,
  useColorMode,
  Alert,
  AlertDescription,
  AlertTitle,
  Flex,
  Box,
  Heading,
  SimpleGrid,
  Select
} from "@chakra-ui/core";
import Graph from "../components/Graph";
import ReferrerTable from "../components/ReferrerTable";
import PageTable from "../components/PageTable";
import GlobalStat from "../components/GlobalStat";
import api, { isLogged } from "../utils/api";
import { errorFetchStats, errorFetchWebsites } from "../utils/messages";
import { UserContext } from "../utils/context";

const defaultStats: Stats = {
  pages: [],
  referrers: [],
  stats: [],
  website: {
    domain: "",
    pageviews: 0,
    users: 0,
    sessions: 0,
    bounceRate: 0,
    avgTime: 0,
    id: null
  }
};

const Home: React.FC = (): ReactElement => {
  const { user } = useContext(UserContext);
  const router = useRouter();
  const toast = useToast();
  const { colorMode } = useColorMode();
  const bg = { light: "gray.50", dark: "gray.900" };
  const color = { light: "grey.900", dark: "gray.50" };

  const [loading, setLoading] = useState(true);
  const [from] = useState(new Date());
  const [websites, setWebsites] = useState([]);
  const [selectedWebsiteId, setSelected] = useState("");
  const [stats, setStats] = useState(defaultStats);

  const { website } = stats;

  // Effect used to fetch stats from a selected domain.
  // Triggered on mount cycle and when user changes domain from
  // the <Select /> component.
  useEffect(() => {
    const fetchStat = async (): Promise<void> => {
      try {
        let start: Date | number = new Date();
        const end = getUnixTime(from);

        start.setFullYear(start.getFullYear() - 1);
        start = getUnixTime(start);

        const { data } = await api.get(
          `/stats?website_id=${selectedWebsiteId}&start=${start}&end=${end}&by=month`
        );
        setStats(data);
      } catch (err) {
        console.error(err);
        toast(errorFetchStats);
      }

      setLoading(false);
    };

    if (selectedWebsiteId) {
      setLoading(true);
      fetchStat();
    }
  }, [selectedWebsiteId, from]);

  // Effect used as ComponentDidMount, retrieves websites from the current user.
  useEffect(() => {
    const call = async (): Promise<void> => {
      const urlParams = new URLSearchParams(window.location.search);
      const queryId = urlParams.get("id") || null;

      try {
        const { data = [] } = await api.get("/websites");
        if (!data.length) {
          return;
        }
        // Set the list of websites (used in the <Select /> component.
        setWebsites(data);
        // If we don't get domain id from the query, we select
        // and get the first one.
        if (!queryId) {
          setSelected(data[0].id);
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

  if (!user) {
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
    <Box>
      <Flex justifyContent="space-between" alignContent="center">
        <Heading as="h1">Dashboard</Heading>

        {!!websites.length && (
          <Select
            width="300px"
            value={websites.find(w => w.id === selectedWebsiteId)?.id}
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
      <Graph data={stats.stats} from={from} loading={loading} />

      <SimpleGrid columns={2} spacing={20}>
        <PageTable loading={loading} pages={stats.pages} />
        <ReferrerTable loading={loading} referrers={stats.referrers} />
      </SimpleGrid>
    </Box>
  );
};

export default Home;

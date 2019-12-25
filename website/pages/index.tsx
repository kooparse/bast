import React, { Component } from "react";
import Head from "next/head";
import config from "next/config";
import groupBy from "lodash/groupBy";
import {
  Box,
  Heading,
  Text,
  DataTable,
  Meter,
  Select,
  Paragraph,
  Stack,
  Table,
  TableBody,
  TableCell,
  TableRow
} from "grommet";
import api, { isLogged } from "../utils/api";
import { UserContext } from "../utils/context";

const { SCRIPT_URL, API_URL } = config().publicRuntimeConfig;

const Axis = ({ values, ...rest }) => (
  <Box justify="between" {...rest}>
    {values.map(v => (
      <Text key={v} size="small" color="text-xweak">
        {v}
      </Text>
    ))}
  </Box>
);

const defaultStats: Stats = {
  pages: [],
  ghosts: [],
  website: {
    domain: "",
    visitors: 0,
    sessions: 0,
    id: null
  }
};

class Home extends Component {
  static contextType = UserContext;

  state = {
    websites: [],
    selected: "",
    stats: defaultStats
  };

  getStats = async (websiteId: number) => {
    try {
      const today = new Date();
      const currentYear = today.getFullYear();
      const startOfYear = new Date(currentYear, 0, 0, 0, 0, 0);

      const { data: stats } = await api.get(
        `/stats?website_id=${websiteId}&start=${+startOfYear}&end=${+today}`
      );
      return stats;
    } catch (e) {
      console.error(e);
    }
  };

  async componentDidMount() {
    if (isLogged()) {
      const { data: websites } = await api.get("/websites");
      const hasWebsite = !!websites.length;

      let state = {
        websites,
        selected: hasWebsite ? websites[0].domain : "",
        stats: defaultStats
      };

      if (hasWebsite) {
        state.stats = await this.getStats(websites[0].id);
      }

      this.setState(state);
    }
  }

  handleChange = async e => {
    const { value: selected } = e;
    const website = this.state.websites.find(w => w.domain === selected);
    const stats = await this.getStats(website.id);
    this.setState({ selected, stats: stats });
  };

  render() {
    const { stats, selected, websites } = this.state;
    const website = websites.find(w => w.domain === selected) || {};

    let scriptString = `
      <script>
        (function() {
          window.__bast__website_id = ${website.id};
          window.__bast__user_id = ${this.context.user.id};
          window.__bast__trackerUrl = "${API_URL}/ghost.png";

          var script = document.createElement('script');
          script.src = "${SCRIPT_URL}";
          script.async = false;
          document.head.appendChild(script);
        })();
      </script>
    `;

    const months = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sept",
      "Oct",
      "Nov",
      "Dec"
    ];

    let generalData = [
      { month: months[0], visits: 0, sessions: 0 },
      { month: months[1], visits: 0, sessions: 0 },
      { month: months[2], visits: 0, sessions: 0 },
      { month: months[3], visits: 0, sessions: 0 },
      { month: months[4], visits: 0, sessions: 0 },
      { month: months[5], visits: 0, sessions: 0 },
      { month: months[6], visits: 0, sessions: 0 },
      { month: months[7], visits: 0, sessions: 0 },
      { month: months[8], visits: 0, sessions: 0 },
      { month: months[9], visits: 0, sessions: 0 },
      { month: months[10], visits: 0, sessions: 0 },
      { month: months[11], visits: 0, sessions: 0 }
    ];

    stats.ghosts.forEach(g => {
      const date = new Date(g.createdAt);
      // Between 0-11.
      const indexMonth = date.getMonth() - 1;

      generalData[indexMonth].visits += 1;
      if (g.isNewSession) {
        generalData[indexMonth].sessions += 1;
      }
    });

    return (
      <div>
        <Head>
          <title>Home</title>
        </Head>

        {!!websites.length && (
          <Box
            direction="row"
            gap="medium"
            justify="center"
            margin={{
              top: "medium",
              bottom: "xlarge"
            }}
          >
            <Select
              size="large"
              options={websites.map(w => w.domain)}
              value={selected}
              onChange={this.handleChange}
            />
          </Box>
        )}

        {!!stats.website.id && (
          <div>
            <Box direction="row" gap="medium" justify="center">
              <Box
                width="small"
                pad="small"
                background={{ color: "light-3" }}
                round="xsmall"
                gap="medium"
              >
                <Heading level={1} margin="none">
                  {stats.website.visitors}
                </Heading>
                <Heading level={3} margin="none">
                  Visitors
                </Heading>
              </Box>
              <Box
                width="small"
                pad="small"
                background={{ color: "light-3" }}
                round="xsmall"
                gap="medium"
              >
                <Heading level={1} margin="none">
                  {stats.website.sessions}
                </Heading>
                <Heading level={3} margin="none">
                  Sessions
                </Heading>
              </Box>
              <Box
                width="small"
                pad="small"
                background={{ color: "light-3" }}
                round="xsmall"
                gap="medium"
              >
                <Heading level={1} margin="none">
                  00:32
                </Heading>
                <Heading level={3} margin="none">
                  Avg time
                </Heading>
              </Box>
            </Box>

            <Box
              fill
              direction="row"
              gap="medium"
              justify="stretch"
              margin="medium"
              round="small"
            >
              <Box
                fill
                pad="small"
                background={{ color: "light" }}
                round="xsmall"
                gap="medium"
              >
                <Table caption="Meter Inside Table">
                  <TableBody>
                    {generalData
                      .map(d => ({
                        ...d,
                        percentVisits: (d.visits / d.sessions + d.visits) * 100,
                        percentSessions:
                          (d.sessions / d.sessions + d.visits) * 100
                      }))
                      .map((val, index) => (
                        <TableRow key={index}>
                          <TableCell>
                            <Meter
                              type="bar"
                              values={[
                                {
                                  value: val.percentVisits
                                },
                                {
                                  value: val.percentSessions
                                }
                              ]}
                            />
                          </TableCell>
                          <TableCell>
                            <Text>{val.month}</Text>
                          </TableCell>
                        </TableRow>
                      ))}
                  </TableBody>
                </Table>

              </Box>
              <Box
                fill
                pad="small"
                background={{ color: "light" }}
                round="xsmall"
                gap="medium"
              >
                <DataTable
                  step={12}
                  size="medium"
                  columns={[
                    {
                      property: "pathname",
                      header: <Text>Pathname</Text>,
                      primary: true,
                      render: datum => (
                        <Text weight="bold">{datum.pathname}</Text>
                      )
                    },
                    {
                      property: "visitors",
                      header: "Visitors"
                    },
                    {
                      property: "sessions",
                      header: "Sessions"
                    },
                    {
                      property: "percent",
                      header: "Ratio",
                      render: datum => (
                        <Box pad={{ vertical: "xsmall" }}>
                          <Meter
                            values={[
                              { value: datum.percentVisits },
                              { value: datum.percentSessions }
                            ]}
                            thickness="medium"
                            size="small"
                          />
                        </Box>
                      )
                    }
                  ]}
                  data={stats.pages.map(p => ({
                    ...p,
                    percentVisits: (p.visitors / p.sessions + p.visitors) * 100,
                    percentSessions:
                      (p.sessions / p.sessions + p.visitors) * 100
                  }))}
                />
              </Box>
            </Box>

            <Paragraph margin="none">{scriptString}</Paragraph>
          </div>
        )}
      </div>
    );
  }
}

export default Home;

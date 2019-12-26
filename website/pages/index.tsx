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
  TableRow,
  Chart,
  TextArea
} from "grommet";
import api, { isLogged } from "../utils/api";
import { getGraphData } from "../utils/data";
import { UserContext } from "../utils/context";

const { SCRIPT_URL, API_URL } = config().publicRuntimeConfig;

const LabelledChart = ({ datum: { month, visits, sessions } }) => (
  <Box basis="xsmall" align="center" gap="small">
    <Chart
      aria-label="chart"
      bounds={[
        [0, 1],
        [0, 60]
      ]}
      type="bar"
      values={[{ value: [1, visits] }]}
      size={{ height: "medium", width: "xxsmall" }}
    />
    <Box align="center">
      <Text weight="bold">{month}</Text>
    </Box>
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

    return (
      <Box margin={{ top: "medium" }}>
        <Head>
          <title>Home</title>
        </Head>

        {!!websites.length && (
          <Box direction="row" gap="medium">
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
            <Box
              direction="row"
              gap="medium"
              margin={{
                top: "xlarge",
                bottom: "large"
              }}
            >
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

            <Box>
              <Box pad="large" direction="row" gap="medium">
                {getGraphData(stats.ghosts).map(d => {
                  return <LabelledChart datum={d} />;
                })}
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

            <Box height="medium">
              <TextArea value={scriptString} resize={false} size="small" fill />
            </Box>
          </div>
        )}
      </Box>
    );
  }
}

export default Home;

import React, { Component } from "react";
import Head from "next/head";
import Router from "next/router";
import config from "next/config";
import {
  Box,
  Heading,
  Text,
  DataTable,
  Meter,
  Select,
  TextArea
} from "grommet";
import Graph from "../components/Graph";
import api, { isLogged } from "../utils/api";
import { getGraphData, getScript } from "../utils/data";
import { UserContext } from "../utils/context";

const { API_URL } = config().publicRuntimeConfig;

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
      const start = new Date();
      start.setFullYear(start.getFullYear() - 1 );

      const { data: stats } = await api.get(
        `/stats?website_id=${websiteId}&start=${+start}&end=${+new Date()}`
      );
      return stats;
    } catch (e) {
      console.error(e);
    }
  };

  async componentDidMount() {
    if (isLogged()) {
      const urlParams = new URLSearchParams(window.location.search);
      const queryId = urlParams.get("id");

      const { data: websites } = await api.get("/websites");
      const hasWebsite = !!websites.length;

      let state = {
        websites,
        selected: "",
        stats: defaultStats
      };

      if (!!queryId) {
        const website = websites.find(w => `${w.id}` === queryId);
        state.stats = await this.getStats(website.id);
        state.selected = website.domain;
      } else if (hasWebsite) {
        state.stats = await this.getStats(websites[0].id);
        state.selected = websites[0].domain;
      }

      this.setState(state);
    }
  }

  handleChange = async e => {
    const { value: selected } = e;
    const website = this.state.websites.find(w => w.domain === selected);
    const stats = await this.getStats(website.id);

    // Replace url state with new id for user refresh.
    const params = new URLSearchParams(location.search);
    params.set("id", website.id);
    window.history.replaceState({}, "", `${location.pathname}?${params}`);

    this.setState({ selected, stats: stats });
  };

  render() {
    const { stats, selected, websites } = this.state;
    const website = websites.find(w => w.domain === selected) || {};
    const script = getScript(this.context.user, website);

    return (
      <Box margin={{ top: "medium" }}>
        <Head>
          <title>Dashboard</title>
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
                  Visits
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
                  uniques
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
              <Graph data={getGraphData(stats.ghosts)} />
              <Box margin={{ vertical: "medium" }} fill>
                <Heading level={2} margin="small">
                  Page stats
                </Heading>
                <Box
                  fill
                  background={{ color: "light" }}
                  round="xsmall"
                  gap="medium"
                >
                  <DataTable
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
                              thickness="small"
                              size="small"
                            />
                          </Box>
                        )
                      }
                    ]}
                    data={stats.pages.map(p => ({
                      ...p,
                      percentVisits:
                        (p.visitors / p.sessions + p.visitors) * 100,
                      percentSessions:
                        (p.sessions / p.sessions + p.visitors) * 100
                    }))}
                  />
                </Box>
              </Box>
            </Box>

            <Box height="medium">
              <TextArea value={script} resize={false} size="small" fill />
            </Box>
          </div>
        )}
      </Box>
    );
  }
}

export default Home;

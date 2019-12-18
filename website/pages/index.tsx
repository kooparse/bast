import React, { Component } from "react";
import Head from "next/head";
import config from "next/config";
import groupBy from "lodash/groupBy";
import api, { isLogged } from "../utils/api";
import "react-vis/dist/style.css";
import {
  XYPlot,
  XAxis,
  VerticalBarSeries,
  AreaSeries,
  LineSeries,
  DiscreteColorLegend
} from "react-vis";
import styled from "styled-components";
import { UserContext } from "../utils/context";

const { SCRIPT_URL, API_URL } = config().publicRuntimeConfig;

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
    if (isLogged) {
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
    const { value: selected } = e.target;

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

    let visitorData = [
      { x: 1, y: 0 },
      { x: 2, y: 0 },
      { x: 3, y: 0 },
      { x: 4, y: 0 },
      { x: 5, y: 0 },
      { x: 6, y: 0 },
      { x: 7, y: 0 },
      { x: 8, y: 0 },
      { x: 9, y: 0 },
      { x: 10, y: 0 },
      { x: 11, y: 0 },
      { x: 12, y: 0 }
    ];

    let sessionData = [
      { x: 1, y: 0 },
      { x: 2, y: 0 },
      { x: 3, y: 0 },
      { x: 4, y: 0 },
      { x: 5, y: 0 },
      { x: 6, y: 0 },
      { x: 7, y: 0 },
      { x: 8, y: 0 },
      { x: 9, y: 0 },
      { x: 10, y: 0 },
      { x: 11, y: 0 },
      { x: 12, y: 0 }
    ];

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

    stats.ghosts.forEach(g => {
      const date = new Date(g.createdAt);
      // Between 0-11.
      const indexMonth = date.getMonth() - 1;

      visitorData[indexMonth].y += 1;
      if (g.isNewSession) {
        sessionData[indexMonth].y += 1;
      }
    });

    return (
      <div>
        <Head>
          <title>Home</title>
        </Head>

        {!!websites.length && (
          <div>
            <select value={selected} onChange={this.handleChange}>
              {websites.map(w => (
                <option key={w.id} value={w.domain}>
                  {w.domain}
                </option>
              ))}
            </select>
            <br />
            <br />
            Script:
            <code> {scriptString}</code>
          </div>
        )}

        {!!stats.website.id && (
          <div>
            <h3>Stats of {stats.website.domain}:</h3>
            <div>total visitors: {stats.website.visitors}</div>
            <div>total sessions: {stats.website.sessions}</div>
            <div
              style={{
                display: "flex",
                justifyContent: "center",
                margin: "40px 0px"
              }}
            >
              <div>
                <DiscreteColorLegend
                  style={{ color: "red" }}
                  colors={["#34A0F2", "#FFCD02"]}
                  items={["Visitors", "Sessions"]}
                  orientation="horizontal"
                  strokeWidth={6}
                />
                <XYPlot height={400} width={1200} stackedBy="y">
                  <VerticalBarSeries
                    cluster="stack 1"
                    data={visitorData}
                    color="#34A0F2"
                  />
                  <VerticalBarSeries
                    cluster="stack 1"
                    data={sessionData}
                    color="#FFCD02"
                  />
                  <XAxis
                    tickTotal={12}
                    tickSizeOuter={0}
                    tickSizeInner={0}
                    tickFormat={function tickFormat(d) {
                      console.log(d, months[d - 1]);
                      return months[d - 1];
                    }}
                  />
                </XYPlot>
              </div>
            </div>

            {!!stats.pages.length && (
              <>
                <br />
                <table>
                  <tbody>
                    <tr>
                      <th>Pathname</th>
                      <th>Visitors</th>
                      <th>Sessions</th>
                    </tr>
                    {stats.pages.map(p => (
                      <tr key={p.id}>
                        <td>{p.pathname}</td>
                        <td>{p.visitors}</td>
                        <td>{p.sessions}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </>
            )}
          </div>
        )}
      </div>
    );
  }
}

export default Home;

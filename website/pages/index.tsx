import React, { Component } from "react";
import Head from "next/head";
import config from "next/config";
import api, { isLogged } from "../utils/api";
import { UserContext } from "../utils/context";

const { SCRIPT_URL, API_URL } = config().publicRuntimeConfig;

const defaultStats = {
  pages: [],
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
        const { data } = await api.get(`/stats?website_id=${websites[0].id}`);
        state.stats = data;
      }

      this.setState(state);
    }
  }

  handleChange = async e => {
    const { value: selected } = e.target;

    const website = this.state.websites.find(w => w.domain === selected);
    const { data: stats } = await api.get(`/stats?website_id=${website.id}`);
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
          window.__bast__trackerUrl = "${API_URL}/collect";

          var script = document.createElement('script');
          script.src = "${SCRIPT_URL}";
          script.async = false;
          document.head.appendChild(script);
        })();
      </script>
    `;

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
            <code>{scriptString}</code>
          </div>
        )}

        {!!stats.website.id && (
          <div>
            <h3>Stats of {stats.website.domain}:</h3>
            <div>total visitors: {stats.website.visitors}</div>
            <div>total sessions: {stats.website.sessions}</div>
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

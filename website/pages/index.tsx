import React, { Component } from "react";
import Head from "next/head";
import api, { isLogged } from "../utils/api";

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
    const { stats } = this.state;

    return (
      <div>
        <Head>
          <title>Home</title>
        </Head>
        <div>
          <select value={this.state.selected} onChange={this.handleChange}>
            {this.state.websites.map(w => (
              <option key={w.id} value={w.domain}>
                {w.domain}
              </option>
            ))}
          </select>
        </div>

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

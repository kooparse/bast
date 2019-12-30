import config from "next/config";
const { API_URL, SCRIPT_URL } = config().publicRuntimeConfig;
import { UserContext } from "./context";

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

export const getGraphData = (ghosts: Ghost[]): GraphDatum[] => {
  let data: GraphDatum[] = months.map(month => ({
    month,
    visits: 0,
    sessions: 0,
    avgTime: 3.4,
    percentVisits: 0,
    percentSessions: 0
  }));

  ghosts.forEach(g => {
    const date = new Date(g.createdAt);
    // Between 0-11.
    const indexMonth = date.getMonth() - 1;

    data[indexMonth].visits += 1;
    if (g.isNewSession) {
      data[indexMonth].sessions += 1;
    }
  });

  return data.map(d => ({
    ...d,
    percentVisits: (d.visits / d.sessions + d.visits) * 100,
    percentSessions: (d.sessions / d.sessions + d.visits) * 100
  }));
};

// Create the script to be copy/past by the user.
export const getScript = (user: User, website: Website): string => {
    return `
      <script>
        (function() {
          window.__bast__website_id = ${website.id};
          window.__bast__user_id = ${user.id};
          window.__bast__trackerUrl = "${API_URL}/ghost.png";

          var script = document.createElement('script');
          script.src = "${SCRIPT_URL}";
          script.async = false;
          document.head.appendChild(script);
        })();
      </script>
    `;

}

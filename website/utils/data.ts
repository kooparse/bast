import config from "next/config";
import { format, compareAsc } from "date-fns";
const { API_URL, SCRIPT_URL } = config().publicRuntimeConfig;
import { UserContext } from "./context";

export const getGraphData = (ghosts: Ghost[]): GraphDatum[] => {
  const groupByDate = {};

  ghosts.forEach(g => {
    const date = new Date(g.createdAt);
    const id = format(date, "M::yyyy");

    if (typeof groupByDate[id] === "undefined") {
      groupByDate[id] = { date, visits: 0, uniques: 0 };
    }

    groupByDate[id].visits += 1;
    if (g.isNewSession) {
      groupByDate[id].uniques += 1;
    }
  });

  const data: GraphDatum[] = Object.values(groupByDate);
  return data.sort((a, b) => compareAsc(a.date, b.date));
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
};

type GraphDatum = {
  visits: number;
  sessions: number;
  month: string;
  percentVisits: number;
  percentSessions: number;
};

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

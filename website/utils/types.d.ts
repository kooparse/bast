type User = {
  id: number;
  email: string;
};

type Website = {
  id: number;
  domain: string;
  pageviews: number;
  users: number;
  sessions: number;
  avgTime: number;
};

type Pageview = {
  id: string;
  createdAt: Date;
  hostname: string;
  href: string;
  pathname: string;
  referrer: string;
  websiteId: number;
  isNew: boolean;
};

type Referrer = {
  name: string;
  count: number;
};

type Page = {
  name: string;
  users: number;
  sessions: number;
  pageviews: number;
};

type MonthStat = {
  users: number;
  sessions: number;
  pageviews: number;
};

type GraphData = {
  string?: MonthStat;
};

type OS = {
  name: string;
  counter: number;
};

type Browser = {
  name: string;
  counter: number;
};

type Category = {
  name: string;
  counter: number;
};

type Stats = {
  website: Website;
  stats: GraphData;
  referrers: Referrer[];
  pages: Page[];
  systems: {
    operatingSystems: OS[];
    browsers: Browser[];
    categories: Category[];
  };
};

type Stat = {
  id: number;
  websiteId: number;
  users: number;
  sessions: number;
  avgTime: number;
  createdAt: Date;
};

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
  bounceRate: number;
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
  createdAt: Date;
};

type Stats = {
  website: Website;
  stats: MonthStat[];
  referrers: Referrer[];
  pages: Page[];
};

type Stat = {
  id: number;
  websiteId: number;
  users: number;
  sessions: number;
  avgTime: number;
  bounceRate: number;
  createdAt: Date;
};

type User = {
  id: number;
  email: string;
};

type Page = {
  id: number;
  pathname: string;
  sessions: number;
  visitors: number;
};

type Website = {
  id: number;
  domain: string;
  sessions: number;
  visitors: number;
};

type Ghost = {
  id: number;
  createdAt: Date;
  hostname: string;
  pathname: string;
  referrer: string;
  userId: number;
  websiteId: number;
  isNewSession: boolean;
};

type Stats = {
  website: Website;
  pages: Page[];
  ghosts: Ghost[];
};

type GraphDatum = {
  visits: number;
  uniques: number;
  date: Date;
};

type ReferrerCount = {
  count: number;
  max: number;
  domain: string;
};


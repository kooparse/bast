type Page = {
  id: number;
  pathname: string;
  sessions: number;
  visitors: number;
};

type Website = {
  id: number,
  domain: string,
  sessions: number,
  visitors: number,
};

type Ghost = {
  id: number,
  createdAt: Date,
  hostname: string,
  pathname: string,
  userId: number,
  websiteId: number,
  isNewSession: boolean,
};

type Stats = {
  website: Website;
  pages: Page[];
  ghosts: Ghost[];
};


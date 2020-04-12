import React from "react";

type UserContext = {
  user: User;
  setUser(user?: User): void;
  loading: boolean,
};

export const UserContext = React.createContext<UserContext>({
  user: null,
  setUser: console.info,
  loading: false,
});

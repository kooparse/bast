import React from "react";

type UserContext = {
  // TODO: Use the correct type here.
  user: User;
  setUser(user?: User): void;
};

export const UserContext = React.createContext<UserContext>({
  user: null,
  setUser: console.log
});

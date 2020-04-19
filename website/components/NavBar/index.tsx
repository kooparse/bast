import React, { ReactElement, useContext, useState } from "react";
import Router from "next/router";
import Link from "next/link";
import { UserContext } from "../../utils/context";
import {
  Flex,
  Heading,
  Box,
  MenuItem,
  Button,
  ButtonGroup,
  useColorMode,
  IconButton,
  Divider,
  Menu,
  MenuList,
  MenuGroup,
  MenuDivider,
  MenuButton,
} from "@chakra-ui/core";

const NavBar: React.FC = (): ReactElement => {
  const ctx = useContext(UserContext);
  const { colorMode, toggleColorMode } = useColorMode();
  const [showBurger] = useState(false);
  const isConnected = !!ctx.user?.id;

  const bg = { light: "white", dark: "gray.800" };
  const color = { light: "grey.800", dark: "white" };

  const logout = (): void => {
    window.localStorage.removeItem("token");
    ctx.setUser(null);
    Router.push("/");
  };

  return (
    <Box
      as="nav"
      padding="1.3em"
      borderBottomWidth="1px"
      bg={bg[colorMode]}
      color={color[colorMode]}
    >
      <Flex width="100%" wrap="wrap" justify="space-between">
        <Flex align="center" mr={5}>
          <Heading as="h1" size="lg">
            <Link href="/">
              <a>Bast</a>
            </Link>
          </Heading>
        </Flex>

        <Flex
          display={{ sm: showBurger ? "block" : "none", md: "flex" }}
          mt={{ base: 4, md: 0 }}
        >
          <IconButton
            variant="ghost"
            aria-label="dark-mode"
            icon={colorMode === "light" ? "moon" : "sun"}
            onClick={(): void => {
              toggleColorMode();
              document.cookie = `isDarkMode=${colorMode === "light"}`;
            }}
          />

          <Divider orientation="vertical" />

          <Box ml={3.5} zIndex={20}>
            {isConnected ? (
              <Menu>
                <MenuButton as={Button}>{ctx.user.email}</MenuButton>
                <MenuList>
                  <MenuGroup title="Profile">
                    <MenuItem
                      onClick={(): void => {
                        Router.push("/settings");
                      }}
                    >
                      Settings
                    </MenuItem>
                    <MenuItem onClick={logout}>Logout</MenuItem>
                  </MenuGroup>
                </MenuList>
              </Menu>
            ) : (
              <ButtonGroup spacing={4}>
                <Button
                  bg="transparent"
                  border="1px"
                  onClick={(): void => {
                    Router.push("/login");
                  }}
                >
                  Login
                </Button>
                <Button
                  bg="transparent"
                  border="1px"
                  onClick={(): void => {
                    Router.push("/register");
                  }}
                >
                  Register
                </Button>
              </ButtonGroup>
            )}
          </Box>
        </Flex>
      </Flex>
    </Box>
  );
};

export default NavBar;

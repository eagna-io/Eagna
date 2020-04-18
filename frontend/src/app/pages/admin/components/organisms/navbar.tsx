import React from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import * as storage from "infra/storage";

import { WhiteBaseColor } from "app/components/color";

export const NavigationBar: React.FC = () => {
  const [isLoggedIn, setIsLoggedIn] = React.useState(
    storage.getAdminAccessToken
  );
  if (!isLoggedIn) {
    return (
      <NavBar>
        <NavBarItem>
          <Link to="/admin/login">ログイン</Link>
        </NavBarItem>
      </NavBar>
    );
  }
  return (
    <NavBar>
      <NavBarItem>
        <Link to="/admin/poll/create">Poll作成フォーム</Link>
      </NavBarItem>
      <NavBarItem>
        <Link to="/admin/poll/resolve">PollResolveフォーム</Link>
      </NavBarItem>
      <NavBarItem>
        <Link to="/admin/contest/create">Contest作成フォーム</Link>
      </NavBarItem>
      <NavBarItem>
        <Link to="/admin/contest/close">Contest Close管理</Link>
      </NavBarItem>
    </NavBar>
  );
};

const NavBar = styled.ul`
  width: 100%;
  padding: 0;
  margin: 0;
`;

const NavBarItem = styled.li`
  padding: 9px 0px 9px 9px;
  font-size: 16px;
  font-weight: 500;
  color: ${WhiteBaseColor.hex};
`;

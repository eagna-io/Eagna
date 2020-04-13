import React from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import {
  WhiteBaseColor
} from "app/components/color";

export const NavigationBar: React.FC = () => {
  const isLoggedIn = true;
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
        <Link to="/admin/create">Poll作成フォーム</Link>
      </NavBarItem>
      <NavBarItem>
        <Link to="/admin/resolve">PollResolveフォーム</Link>
      </NavBarItem>
    </NavBar>
  );
}

const NavBar = styled.ul`
  width: 100%;
  padding: 0;
  margin: 0;
`;

const NavBarItem = styled.li`
  font-size: 16px;
  font-weight: 500;
  color: ${WhiteBaseColor.hex};
  padding: 9px 0px 9px 9px;
`;

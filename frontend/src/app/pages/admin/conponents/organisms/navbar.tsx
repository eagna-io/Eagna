import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor
} from "app/components/color";

export const NavigationBar: React.FC = () => {
  return (
    <NavBar>
      <NavBarItem>Poll作成フォーム</NavBarItem>
      <NavBarItem>PollResolveフォーム</NavBarItem>
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

import React from "react";
import styled from "styled-components";

import {
  AdminBackgroundColor,
  AdminMainColor,
  WhiteBaseColor,
  BlackColor
} from "app/components/color";

import { NavigationBar } from "./conponents/organisms/navbar";
import { ResolveItem } from "./conponents/organisms/resolveItem";

export const ResolvePoll: React.FC = () => {
  return (
    <Container>
      <NavBarComponent>
        <NavigationBar />
      </NavBarComponent>
      <Content>
        <PollTitle>{pollTitle}</PollTitle>
        <ResolveContainer>
          <ResolveItem choiceItem="LeBron" pollTitle={pollTitle} />
          <ResolveItem choiceItem="Kobe Bean Bryant" pollTitle={pollTitle} />
        </ResolveContainer>
      </Content>
    </Container>
  );
}

const pollTitle = "次にポイントを決めるのは誰？";

const Container = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: ${AdminBackgroundColor.hex};
  user-select: none;
  display: flex;
`;

const NavBarComponent = styled.div`
  width: 250px;
  height: 100vh;
  background-color: ${AdminMainColor.hex};
  padding-top: 30px;
`;

const Content = styled.div`
  width: 1142px;
  height: calc(100vh - 40px);
  margin: 20px 24px;
  background-color: ${WhiteBaseColor.hex};
  box-shadow: 0 1px 4px 0 ${BlackColor.rgba(0.5)};
  padding: 121px 282px 0 121px;
`;

const PollTitle = styled.div`
  font-size: 21px;
  font-weight: 500;
  color: ${AdminMainColor.hex};
`;

const ResolveContainer = styled.div`
  margin: 70px 0px 0px 95px;
`;

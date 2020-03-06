import React from "react";
import styled from "styled-components";

import Header from "./components/organisms/header";
import Contents from "./components/organisms/contents";
import ChartContainer from "./components/organisms/chartContainer";

const Page: React.FC = () => {

  return (
    <Container>
      <ChartContainer />
      <SubContainer>
        <Header userName="Yuya_F" />
        <Contents
          marketTitle={marketTitle}
          ranking={ranking}
          paticipantsNum={paticipantsNum}
        />
      </SubContainer>
    </Container>
  );
};

export default Page;

const marketTitle = 'RAGE Shadowverse 2020 Spring'
const ranking = 2
const paticipantsNum = 358

const Container = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: rgba(36, 36, 35);
`;

const SubContainer = styled.div`
  width: 100vw;
  background-color: rgba(36, 36, 35, 0.5);
`;

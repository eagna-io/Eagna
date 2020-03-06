import React from "react";
import styled from "styled-components";

import Header from "./components/organisms/header";
import Contents from "./components/organisms/contents";

const Page: React.FC = () => {

  return (
    <Container>
      <Header userName="Yuya_F" />
      <Contents
        marketTitle={marketTitle}
        ranking={ranking}
        paticipantsNum={paticipantsNum}
      />
    </Container>
  );
};

export default Page;

const marketTitle = 'RAGE Shadowverse 2020 Spring'
const ranking = 2
const paticipantsNum = 358

const Container = styled.div`
  width: 100vw;
  background-color: #242423;
`;

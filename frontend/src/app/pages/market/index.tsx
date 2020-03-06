import React from "react";
import styled from "styled-components";

import Header from "./components/organisms/header";
import Contents from "./components/organisms/contents";

const Page: React.FC = () => {

  return (
    <Container>
      <Header userName="Yuya_F" />
      <Contents marketTitle={marketTitle} />
    </Container>
  );
};

export default Page;

const marketTitle = 'RAGE Shadowverse 2020 Spring'

const Container = styled.div`
  width: 100vw;
  background-color: #242423;
`;

import React from "react";
import styled from "styled-components";

import Header from "./components/header";

const Page: React.FC = () => {

  return (
    <Container>
      <Header userName="Yuya_F" />
    </Container>
  );
};

export default Page;

const Container = styled.div`
  width: 100vw;
  background-color: #242423;
`;

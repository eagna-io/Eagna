import React from "react";
import styled from "styled-components";

import Header from "./components/header";
import Contents from "./components/contents";

const Page: React.FC = () => {

  return (
    <Container>
      <Header userName="Yuya_F" />
			<Contents />
    </Container>
  );
};

export default Page;

const Container = styled.div`
  width: 100vw;
  background-color: #242423;
`;

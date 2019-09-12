import React from "react";
import styled from "styled-components";

import { pc } from "app/components/responsive";
import TwitterFeed from "app/components/twitter";

export default () => (
  <Container>
    <Title>お知らせ</Title>
    <TwitterFeed />
  </Container>
);

const Container = styled.div`
  width: 100%;
  margin-top: 50px;

  ${pc(`
    margin-top: 90px;
  `)}
`;

const Title = styled.h3`
  margin: 0;
  padding: 0;
  font-size: 15px;
  font-weight: bold;
`;

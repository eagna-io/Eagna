import React from "react";
import styled from "styled-components";

import { ResolvedMarketColor } from "app/components/color";
import { pc } from "app/components/responsive";

export default () => (
  <Container>
    <Mark src="/img/market/resolved_mark.svg" />
  </Container>
);

const Container = styled.div`
  position: absolute;
  top: 0;
  right: 15px;
  width: 21px;
  height: 29px;
  background-color: ${ResolvedMarketColor.hex};

  ${pc(`
    right: 27px;
    width: 35px;
    height: 48px;
  `)}
`;

const Mark = styled.img`
  display: block;
  width: 14px;
  margin: 5px auto 0 auto;

  ${pc(`
    width: 24px;
    margin: 8px auto 0 auto;
  `)}
`;

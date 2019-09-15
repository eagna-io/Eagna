import React from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import { Market } from "models/market";
import { pc } from "app/components/responsive";
import StatusBadge from "app/components/status_badge";

interface Props {
  market: Market;
}

export default ({ market }: Props) => {
  return (
    <MarketContainer to={`/market/${market.id}`}>
      <MarketImage src={market.attrs.tokens[0].sumbnailUrl} />
      <MarketStatusBadge status={market.status} />
      <MarketTitle>{market.attrs.title}</MarketTitle>
    </MarketContainer>
  );
};

const MarketContainer = styled(Link)`
  display: inline-block;
  width: calc(50% - 6px);
  height: 220px;
  box-shadow: 0 0 2px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;
  margin-top: 20px;
  vertical-align: top;
  overflow: hidden;

  &:nth-of-type(odd) {
    margin-right: 12px;
  }

  ${pc(`
    width: 250px;
    height: 270px;
    margin-left: 50px;
    white-space: normal;

    &:first-of-type {
      margin-left: 0px;
    }

    /* スマホの設定を打ち消す */
    &:nth-of-type(odd) {
    }
  `)}
`;

const MarketImage = styled("div")<{ src: string }>`
  display: block;
  width: 100%;
  height: 120px;
  background-image: url(${props => props.src});
  background-position: center;
  background-size: cover;

  ${pc(`
    height: 150px;
  `)}
`;

const MarketStatusBadge = styled(StatusBadge)`
  margin-top: 10px;
  margin-left: 6px;
  width: 70px;
  height: 18px;
  font-size: 10px;
  font-weight: normal;
  line-height: 18px;
`;

const MarketTitle = styled.h4`
  margin: 10px 0;
  padding: 0 5px;
  font-size: 12px;
  font-weight: bold;

  ${pc(`
    font-size: 16px;
  `)}
`;

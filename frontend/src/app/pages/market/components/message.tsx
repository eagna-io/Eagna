import React from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";
import { useSelector } from 'react-redux';

import { MarketStatus } from "models/market";
import { RootState } from 'app/redux';
import { pc } from "app/components/responsive";
import {
  Color,
  UpcomingMarketColor,
  OpenMarketColor,
  ClosedMarketColor,
  ResolvedMarketColor
} from "app/components/color";

import { useMarket } from "./data_provider";

export default () => {
  const user = useSelector((state: RootState) => state.user.user);
  const { market, myHistory } = useMarket();

  switch (market.status) {
    case MarketStatus.Upcoming:
      return (
        <Container bgcolor={UpcomingMarketColor}>
          マーケットがOpenすると取引を行えるようになります。 Openまでお待ちください。
        </Container>
      );
    case MarketStatus.Open:
      if (user === null) {
        return (
          <Container bgcolor={OpenMarketColor}>
            取引を行うためには<Link to="/login">ログイン</Link>
            する必要があります。
          </Container>
        );
      } else if (user === undefined) {
        return null;
      } else if (!myHistory) {
        return (
          <Container bgcolor={OpenMarketColor}>
            「参加する」ボタンを押すことでコインが配布され 取引を開始できます。
          </Container>
        );
      } else {
        return null;
      }
    case MarketStatus.Closed:
      return (
        <Container bgcolor={ClosedMarketColor}>
          マーケットはすでにCloseしています。結果の決定までお待ちください。
        </Container>
      );
    case MarketStatus.Resolved:
      return (
        <Container bgcolor={ResolvedMarketColor}>
          マーケットはすでにCloseしています。
        </Container>
      );
  }
};

const Container = styled("div")<{ bgcolor: Color }>`
  width: 100%;
  padding: 20px 30px;
  background-color: ${props => props.bgcolor.hex};
  font-size: 11px;
  font-weight: bold;
  line-height: 22px;
  color: white;
  letter-spacing: 1px;

  ${pc(`
    padding: 20px calc((100% - 980px) / 2);
    font-size: 13px;
  `)}

  & a {
    margin: 0 5px;
    text-decoration: underline;
  }
`;

import React from "react";
import styled from "styled-components";
import { useSelector, useDispatch } from "react-redux";

import { RootState } from "app/redux";
import { queryPrizeList } from "app/redux/prize";
import Header from "app/components/header";
import { Container as ResponsiveContainer } from "app/components/responsive";
import UserPoint from "./components/user_point";
import PrizeItem from "./components/prize";

export default () => {
  const prizeList = useSelector((state: RootState) => state.prize.list);
  const dispatch = useDispatch();

  React.useEffect(() => {
    if (prizeList === undefined) {
      dispatch(queryPrizeList());
    }
  }, [prizeList, dispatch]);

  return (
    <>
      <Header />
      <ContentHeader>
        <ContentHeaderTitle>賞品一覧</ContentHeaderTitle>
        <ContentHeaderDesc>
          賞品はポイントで交換することができます。ポイントはマーケットに参加し予測をすることで獲得することができます。
        </ContentHeaderDesc>
        <UserPoint />
      </ContentHeader>
      <PrizeListContainer>
        {prizeList ? prizeList.map(prize => <PrizeItem prize={prize} />) : null}
      </PrizeListContainer>
    </>
  );
};

const ContentHeader = styled(ResponsiveContainer)`
  height: 210px;
  padding-top: 30px;
  background-color: #f6f8fa;
  border-bottom: 1px solid #979797;
`;

const ContentHeaderTitle = styled.h2`
  width: 100%;
  margin: 0;
  font-size: 21px;
  font-weight: bold;
`;

const ContentHeaderDesc = styled.p`
  width: 100%;
  margin-top: 15px;
  font-size: 12px;
  font-weight: normal;
`;

const PrizeListContainer = styled(ResponsiveContainer)`
  margin-top: 60px;
`;

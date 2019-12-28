import React from "react";
import styled from "styled-components";
import { useSelector, useDispatch } from "react-redux";
import Container from "@material-ui/core/Container";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";

import { RootState } from "app/redux";
import { queryPrizeList } from "app/redux/prize";
import Header from "app/components/header";
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
        <Container>
          <ContentHeaderTitle variant="h5">賞品一覧</ContentHeaderTitle>
          <ContentHeaderDesc variant="body2">
            賞品はポイントで交換することができます。ポイントはマーケットに参加し予測をすることで獲得することができます。
          </ContentHeaderDesc>
          <UserPoint />
        </Container>
      </ContentHeader>
      <PrizeListContainer>
        <Grid container spacing={1}>
          {prizeList
            ? prizeList.map(prize => <PrizeItem prize={prize} key={prize.id} />)
            : null}
        </Grid>
      </PrizeListContainer>
    </>
  );
};

const ContentHeader = styled.div`
  width: 100%;
  padding-top: 30px;
  background-color: #f6f8fa;
  border-bottom: 1px solid #979797;
`;

const ContentHeaderTitle = styled(Typography)`
  font-weight: bold;
`;

const ContentHeaderDesc = styled(Typography)`
  margin-top: 15px;
`;

const PrizeListContainer = styled(Container)`
  margin-top: 100px;
  padding-bottom: 50px;
`;

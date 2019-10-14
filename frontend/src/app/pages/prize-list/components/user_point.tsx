import React from "react";
import styled from "styled-components";
import { useSelector } from "react-redux";
import { RootState } from "app/redux";
import Card from "@material-ui/core/Card";
import CardHeader from "@material-ui/core/CardHeader";
import CardContent from "@material-ui/core/CardContent";
import CardActionArea from "@material-ui/core/CardActionArea";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";

export default () => {
  const user = useSelector((state: RootState) => state.user.user);
  return (
    <StyledCard>
      <CardHeader
        title="保有ポイント"
        titleTypographyProps={{ variant: "subtitle1" }}
      />
      <CardContent>
        <Grid container alignItems="center">
          <Grid item xs={4}>
            <Icon src="/img/prize/trophy.svg" />
          </Grid>
          <Grid item xs={8}>
            <Point variant="h6" align="right">
              {user ? user.point : "-"} ポイント
            </Point>
          </Grid>
        </Grid>
      </CardContent>
      <CardFooter>
        <Typography variant="caption">
          ポイント獲得・使用履歴を確認する
        </Typography>
      </CardFooter>
    </StyledCard>
  );
};

const StyledCard = styled(Card)`
  position: relative;
  width: 100%;
  max-width: 400px;
  top: 50px;
`;

const Icon = styled.img`
  display: block;
  margin: 0 auto;
  width: 40px;
  height: 40px;
`;

const Point = styled(Typography)`
  font-weight: bold;
`;

const CardFooter = styled(CardActionArea)`
  padding: 6px 11px;
  border-top: solid 1px #d1d5da;
`;

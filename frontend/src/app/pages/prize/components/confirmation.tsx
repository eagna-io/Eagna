import React from "react";
import styled from "styled-components";
import Card from "@material-ui/core/Card";
import CardHeader from "@material-ui/core/CardHeader";
import CardContent from "@material-ui/core/CardContent";
import CardActions from "@material-ui/core/CardActions";
import Button from "@material-ui/core/Button";
import Grid from "@material-ui/core/Grid";
import Typography from "@material-ui/core/Typography";

import { EagnaColor } from "app/components/color";
import { Prize } from "models/prize";

interface Props {
  prize: Prize;
  closeModal: () => void;
}

export default ({ prize, closeModal }: Props) => {
  return (
    <StyledCard raised>
      <CardHeader
        title={
          <Title align="center" variant="subtitle1">
            賞品とポイントを交換
          </Title>
        }
      />
      <CardContent>
        <Typography variant="body2" paragraph>
          <Bold>{prize.price}ポイント</Bold>と
          <br />
          <Bold>{prize.name}</Bold>
          <br />
          の交換をリクエストしますか？
        </Typography>
        <Typography variant="caption" color="textSecondary">
          リクエストを送信してから、賞品の受け取りまで数日かかる場合があります。賞品ページをご確認ください。
        </Typography>
      </CardContent>
      <CardActions>
        <Grid container justify="space-evenly">
          <NoButton variant="contained" size="large" onClick={closeModal}>
            いいえ
          </NoButton>
          <YesButton variant="contained" size="large">
            はい
          </YesButton>
        </Grid>
      </CardActions>
    </StyledCard>
  );
};

const StyledCard = styled(Card)`
  width: 300px;
  margin: 25vh auto 0 auto;
  padding: 10px 5px 20px 5px;
`;

const Title = styled(Typography)`
  font-weight: bold;
`;

const Bold = styled.span`
  font-weight: bold;
`;

const NoButton = styled(Button)``;

const YesButton = styled(Button)`
  background-color: ${EagnaColor.hex};
  color: white;
`;

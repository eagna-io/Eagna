import React from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";
import Grid from "@material-ui/core/Grid";
import Card from "@material-ui/core/Card";
import CardActionArea from "@material-ui/core/CardActionArea";
import CardMedia from "@material-ui/core/CardMedia";
import CardContent from "@material-ui/core/CardContent";
import Typography from "@material-ui/core/Typography";

import { Prize } from "models/prize";

interface Props {
  prize: Prize;
}

export default ({ prize }: Props) => {
  return (
    <Grid item xs={6} sm={2}>
      <Card>
        <Link to={`/prize/${prize.id}`}>
          <CardActionArea>
            <CardThumbnail image={prize.thumbnailUrl} />
            <CardContent>
              <Typography gutterBottom variant="body2">
                {prize.name}
              </Typography>
              <Point variant="body2" align="right">
                {prize.price} ポイント
              </Point>
            </CardContent>
          </CardActionArea>
        </Link>
      </Card>
    </Grid>
  );
};

const CardThumbnail = styled(CardMedia)`
  height: 170px;
`;

const Point = styled(Typography)`
  font-weight: bold;
  color: #f9aa33;
`;

import React from "react";
import styled from "styled-components";
import { useSelector, useDispatch } from "react-redux";
import Container from "@material-ui/core/Container";
import Grid from "@material-ui/core/Grid";
import CircularProgress from "@material-ui/core/CircularProgress";
import Typography from "@material-ui/core/Typography";
import Paper from "@material-ui/core/Paper";
import Button from "@material-ui/core/Button";
import Modal from "@material-ui/core/Modal";
import Backdrop from "@material-ui/core/Backdrop";
import Fade from "@material-ui/core/Fade";
import ReactMarkdown from "react-markdown";

import { RootState } from "app/redux";
import { queryPrizeList } from "app/redux/prize";
import { Prize } from "models/prize";
import { EagnaColor } from "app/components/color";
import Header from "app/components/header";
import ConfirmationComponent from "./components/confirmation";

interface Props {
  prizeId: string;
}

export default ({ prizeId }: Props) => {
  const prizeList = useSelector((state: RootState) => state.prize.list);
  const dispatch = useDispatch();

  React.useEffect(() => {
    if (prizeList === undefined) {
      dispatch(queryPrizeList());
    }
  }, [prizeList, dispatch]);

  const prize: Prize | undefined = prizeList
    ? prizeList.find(prize => prize.id === prizeId)
    : undefined;

  return (
    <>
      <Header />
      <StyledContainer maxWidth="md">
        {prize ? <PrizeContent prize={prize} /> : <CircularProgress />}
      </StyledContainer>
    </>
  );
};

const PrizeContent: React.FC<{ prize: Prize }> = ({ prize }) => {
  const [showModal, setShowModal] = React.useState(false);
  const user = useSelector((state: RootState) => state.user.user);

  return (
    <>
      <Grid container spacing={4} justify="space-between">
        <Grid item xs={12} sm={5}>
          <StyledPaper>
            <Thumbnail src={prize.thumbnailUrl} alt="prize thumbnail" />
          </StyledPaper>
        </Grid>
        <Grid item xs={12} sm={6}>
          <Name variant="h6">{prize.name}</Name>
          <Point variant="subtitle1" align="right">
            {prize.price} ポイント
          </Point>
          <Typography variant="body2" align="right" color="textSecondary">
            {user ? user.point : "-"} ポイント
            <Typography variant="caption">持っています</Typography>
          </Typography>
          <RequestButton
            variant="contained"
            fullWidth
            size="large"
            onClick={() => setShowModal(true)}
            disabled={!user || user.point < prize.price}
          >
            交換をリクエストする
          </RequestButton>
          {!user ? (
            <Typography variant="caption" color="error">
              ポイントと賞品を交換するにはログインが必要です。
            </Typography>
          ) : user.point < prize.price ? (
            <Typography variant="caption" color="error">
              ポイントが不足しています
            </Typography>
          ) : null}
        </Grid>
        <Grid item xs={12}>
          <Description source={prize.description} linkTarget="_blank" />
        </Grid>
      </Grid>
      <Modal
        open={showModal}
        onClose={() => setShowModal(false)}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500
        }}
      >
        <Fade in={showModal}>
          <ConfirmationComponent
            prize={prize}
            closeModal={() => setShowModal(false)}
          />
        </Fade>
      </Modal>
    </>
  );
};

const StyledContainer = styled(Container)`
  padding-top: 5vh;
  padding-bottom: 30px;
`;

const StyledPaper = styled(Paper)`
  overflow: hidden;
`;

const Thumbnail = styled.img`
  display: block;
  width: 100%;
`;

const Name = styled(Typography)`
  font-weight: bold;
`;

const Point = styled(Typography)`
  color: #f9aa33;
`;

const RequestButton = styled(Button)`
  margin-top: 20px;
  background-color: ${EagnaColor.hex};
  color: white;
  font-weight: bold;
`;

const Description = styled(ReactMarkdown)`
  width: 100%;
  & a {
    text-decoration: underline;
  }
`;

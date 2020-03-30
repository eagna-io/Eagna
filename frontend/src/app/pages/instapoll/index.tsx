import React from "react";
import styled from "styled-components";
import moment from "moment";

import {
  BackgroundMainColor,
  WhiteBaseColor,
  ChoiceBlue,
  MainRed,
  TextBaseColor,
  BlackColor
} from "app/components/color";

import { Timer } from "./components/organisms/timer";
import { CommentCard } from "./components/organisms/commentCard";
import { ChoiceList } from "./components/organisms/choiceList";
import { ReactComponent as SubmitIcon } from "./components/atoms/images/send.svg";
import * as ws from "./infra/ws";
import { reducer, initialState } from "./reducer";

export const InstapollPage: React.FC = () => {
  const [state, dispatch] = React.useReducer(reducer, initialState);
  const { poll, comments, timer } = state;

  // Websocketコネクションを確立する
  React.useEffect(() => {
    ws.open({
      onComment: comment => {
        dispatch({ type: "pushComment", comment });
      },
      onPoll: poll => {
        dispatch({ type: "updatePoll", poll });
      }
    });
  }, []);

  // 一定間隔でtickアクションを送る
  React.useEffect(() => {
    const timer = setInterval(() => {
      dispatch({ type: "tick", time: moment() });
    }, 950);

    return () => {
      clearInterval(timer);
    };
  }, []);

  if (poll && timer) {
    return (
      <Container>
        <Timer content={timer} />
        <CommentFeed>
          {comments.map(comment => (
            <CommentCard comment={comment} />
          ))}
        </CommentFeed>
        <PollCard>
          <Theme>{poll.title}</Theme>
          <ChoiceList />
          <CommentContainer>
            <CommentInput type="text" placeholder="コメントする" />
            <Submit />
          </CommentContainer>
        </PollCard>
      </Container>
    );
  } else {
    return <Container>Loading...</Container>;
  }
};

const themeTitle = "次にポイントを決めるのは誰？";

const Container = styled.div`
  width: 100vw;
  height: calc(100vh - 75px);
  padding: 8px 28px;
  background-color: ${BackgroundMainColor.hex};
  user-select: none;
  position: relative;
`;

const CommentFeed = styled.div`
  width: 100%;
  height: 50%;
`;

const PollCard = styled.div`
  position: absolute;
  height: 244px;
  bottom: 20px;
  width: calc(100vw - 56px);
  border-radius: 4px;
  padding: 8px 13px;
  background-color: ${WhiteBaseColor.hex};
  box-shadow: 0 2px 4px 0 ${BlackColor.rgba(0.5)};
`;

const Theme = styled.div`
  width: 100%;
  font-size: 16px;
  margin-bottom: 24px;
`;

const CommentContainer = styled.div`
  width: 100%;
  display: flex;
`;

const CommentInput = styled.input`
  width: 86%;
  border-radius: 24px;
  height: 30px;
  border: solid 1px ${TextBaseColor.hex};
  margin-right: 8px;
  padding: 0px 16px;
  font-size: 16px;
  line-height: 30px;
  transform: scale(0.95);
`;

const Submit = styled(SubmitIcon)`
  width: 32px;
  height: 28px;
`;

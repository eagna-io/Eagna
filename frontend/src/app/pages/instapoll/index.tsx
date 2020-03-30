import React from "react";
import styled from "styled-components";

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

export const InstapollPage: React.FC = () => {
  return (
    <Container>
      <Timer content={timerState} />
      <CommentFeed>
        {comments.map(comment => (
          <CommentCard comment={comment} />
        ))}
      </CommentFeed>
      <PollCard>
        <Theme>{themeTitle}</Theme>
        <ChoiceList />
        <CommentContainer>
          <CommentInput type="text" placeholder="コメントする"></CommentInput>
          <Submit></Submit>
        </CommentContainer>
      </PollCard>
    </Container>
  );
};

const timerState = 123;
const themeTitle = "次にポイントを決めるのは誰？";
const comments = [
  {
    user: "Yuya_F",
    comment: "レブロン調子いいね",
    color: MainRed.hex
  },
  {
    user: "Yuya_F",
    comment: "レブロン調子いいね",
    color: ChoiceBlue.hex
  },
  {
    user: "Yuya_F",
    comment: "レブロン調子いいね",
    color: ChoiceBlue.hex
  },
  {
    user: "Yuya_F",
    comment: "レブロン調子いいね",
    color: MainRed.hex
  }
];

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

import React from "react";
import styled from "styled-components";

import {
  BackgroundMainColor,
  WhiteBaseColor,
  TextBaseColor,
  BlackColor
} from "app/components/color";
import { Poll, Comment, Timer } from "model/poll";

import { Timer as TimerComponent } from "./components/organisms/timer";
import { CommentCard } from "./components/organisms/commentCard";
import { ChoiceList } from "./components/organisms/choiceList";
import { ReactComponent as SubmitIcon } from "./components/atoms/images/send.svg";

interface Props {
  account: string;
  poll: Poll;
  comments: Comment[];
  timer: Timer;
  ws: WebSocket;
}

export const Page: React.FC<Props> = ({
  account,
  poll,
  comments,
  timer,
  ws
}) => {
  const [commentInput, setCommentInput] = React.useState("");
  const [selected, setSelected] = React.useState<string | undefined>();

  return (
    <Container>
      <TimerComponent content={timer} />
      <CommentFeed>
        {comments.map(comment => (
          <CommentCard comment={comment} />
        ))}
      </CommentFeed>
      <PollCard>
        <Theme>{poll.title}</Theme>
        <ChoiceList
          poll={poll}
          selected={selected}
          onSelected={choice => {
            ws.send(
              JSON.stringify({
                type: "updateChoice",
                account,
                choice
              })
            );
            setSelected(choice);
          }}
        />
        <CommentContainer>
          <CommentInput
            type="text"
            placeholder="コメントする"
            value={commentInput}
            onChange={e => setCommentInput(e.target.value)}
          />
          <Submit
            onClick={() => {
              if (commentInput) {
                ws.send(
                  JSON.stringify({
                    type: "addComment",
                    account,
                    comment: commentInput
                  })
                );
                setCommentInput("");
              }
            }}
          />
        </CommentContainer>
      </PollCard>
    </Container>
  );
};

export const LoadingPage: React.FC = () => {
  return <Container>Loading...</Container>;
};

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
  cursor: pointer;
`;

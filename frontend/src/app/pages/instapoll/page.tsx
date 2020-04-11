import React from "react";
import styled from "styled-components";

import {
  WildWatermelon,
  ToreaBay,
  WhiteBaseColor,
  TextBaseColor,
  BlackColor
} from "app/components/color";
import { Poll, Comment, Timer } from "model/poll";

import { Timer as TimerComponent } from "./components/organisms/timer";
import { Score } from "./components/organisms/score";
import { CommentCard } from "./components/organisms/commentCard";
import { ChoiceList } from "./components/organisms/choiceList";
import { ReactComponent as SubmitIcon } from "./components/atoms/images/send.svg";
import { ReactComponent as LogoIcon } from "./components/atoms/images/PlayPoll_logo_white.svg";

interface Props {
  account: string;
  poll: Poll;
  comments: Comment[];
  timer: Timer;
  ws?: WebSocket;
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
      <Header>
        <Logo>LOGO</Logo>
        <TimerComponent content={timer} />
        <Score numerator={2} denominator={3} />
      </Header>
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
            if(ws) {
              ws.send(
                JSON.stringify({
                  type: "updateChoice",
                  account,
                  choice
                })
              );
              setSelected(choice);
            }
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
                if (ws) {
                  ws.send(
                    JSON.stringify({
                      type: "addComment",
                      account,
                      comment: commentInput
                    })
                  )
                }
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
  padding: 16px 28px;
  background-image: linear-gradient(151deg, ${WildWatermelon.hex} 0%, ${ToreaBay.hex} 100%);
  user-select: none;
  position: relative;
`;

const Header = styled.div`
  width: 100%;
  display: flex;
  justify-content: flex-end;
  margin-bottom: 16px;
`;

const Logo = styled(LogoIcon)`
  width: 136px;
  height: 31px;
  margin-top: 8px;
  margin-right: auto;
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

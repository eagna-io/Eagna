import React from "react";
import styled from "styled-components";

import {
  WildWatermelon,
  ToreaBay,
  WhiteBaseColor,
  BlackColor
} from "app/components/color";
import { Poll, Comment, Timer } from "model/poll";

import { Timer as TimerComponent } from "./components/organisms/timer";
import { Score } from "./components/organisms/score";
import { CommentCard } from "./components/organisms/commentCard";
import { ChoiceList } from "./components/organisms/choiceList";
import { ResultModal } from "./components/organisms/resultModal";
import { ContestBoard } from "./components/molecules/contestMessage";
import { ReactComponent as SubmitIcon } from "./components/atoms/images/send.svg";
import { ReactComponent as LogoIcon } from "./components/atoms/images/PlayPoll_logo_white.svg";

interface Props {
  account: string;
  poll?: Poll;
  comments: Comment[];
  timer: Timer;
  ws?: WebSocket;
  contest: "upcoming" | "open" | "closed" | "archived";
}

export const Page: React.FC<Props> = ({
  account,
  poll,
  comments,
  timer,
  ws,
  contest
}) => {
  const [commentInput, setCommentInput] = React.useState("");
  const [selected, setSelected] = React.useState<string | undefined>();
/* 
【MEMO：正誤モーダル表示の方針】
pollがresolveされていない => null
pollがresolve && 正解 => CorectModal
pollがresolve && 不正解 => WrongModal
*/
  if (poll === undefined || contest === "closed"){
    return (
      <Container>
        <Header>
          <Logo />
          <TimerComponent content={timer} />
          <Score numer={2} denom={3} />
        </Header>
        <PollCard>
          { contest==="upcoming" ? <ContestBoard contest={"upcoming"} startAt={"06.01 11:00〜"} /> : null }
          { contest==="open" ? <ContestBoard contest={"open"} /> : null }
          { contest==="closed" || contest==="archived" ? <ContestBoard contest={contest} numer={2} denom={3} /> : null }
        </PollCard>
        <CommentFeed small={false}>
          {comments.map(comment => (
            <CommentCard comment={comment} />
          ))}
        </CommentFeed>
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
      </Container>
    );
  } else {
    return (
      <Container>
        { poll.resolved !== undefined && poll.selected === poll.resolved ? <ResultModal isCorrect={true} /> : null }
        { poll.resolved !== undefined && poll.selected !== poll.resolved ?<ResultModal isCorrect={false} /> : null }
        <Header>
          <Logo />
          <TimerComponent content={timer} />
          <Score numer={2} denom={3} />
        </Header>
        <PollCard>
          { poll.status === "open" ? 
            <Theme><PollIndex>Q{poll.idx}.</PollIndex>{poll.title}</Theme> : null
          }
          <ChoiceList
            poll={poll}
            selected={poll.selected}
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
        </PollCard>
        <CommentFeed small={true}>
          {comments.map(comment => (
            <CommentCard comment={comment} />
          ))}
        </CommentFeed>
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
      </Container>
    );
  }
};

export const LoadingPage: React.FC = () => {
  return <Container>Loading...</Container>;
};

const Container = styled.div`
  position: relative;
  width: 100vw;
  height: calc(100vh - 75px);
  padding: 16px 28px;
  background-image: linear-gradient(151deg, ${WildWatermelon.hex} 0%, ${ToreaBay.hex} 100%);
  user-select: none;
`;

const Header = styled.div`
  display: flex;
  justify-content: flex-end;
  width: 100%;
  margin-bottom: 16px;
`;

const Logo = styled(LogoIcon)`
  width: 136px;
  height: 31px;
  margin-top: 8px;
  margin-right: auto;
`;

const PollCard = styled.div`
  border-radius: 4px;
  padding: 24px 14px;
  margin-bottom: 20px;
  background-color: ${WhiteBaseColor.hex};
  box-shadow: 0 24px 24px 0 ${BlackColor.rgba(0.3)}, 0 0 24px 0 ${BlackColor.rgba(0.22)};
`;

const Theme = styled.div`
  width: 100%;
  margin-bottom: 30px;
  font-size: 16px;
  font-weight: bold;
`;

const PollIndex = styled.span`
  margin-right: 4px;
`;

const CommentFeed = styled.div<{ small: boolean }>`
  height: ${ props => props.small ? "208px" : "336px"};
  margin: 0 14px 18px 14px;
  overflow: scroll;
`;

const CommentContainer = styled.div`
  display: flex;
  position: absolute;
  bottom: 16px;
  width: calc(100vw - 56px);
`;

const CommentInput = styled.input`
  width: 86%;
  height: 30px;
  margin-right: 8px;
  padding: 0px 16px;
  border: solid 1px ${WhiteBaseColor.hex};
  border-radius: 4px;
  font-size: 16px;
  line-height: 30px;
  transform: scale(0.95);
  background-color:transparent;
  ::placeholder {
    color: ${WhiteBaseColor.hex};
    font-size: 14px;
  }
`;

const Submit = styled(SubmitIcon)`
  width: 32px;
  height: 28px;
  cursor: pointer;
`;

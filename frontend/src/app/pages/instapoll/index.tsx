import React from "react";
import styled from "styled-components";

import {
  BackgroundMainColor,
  ShadowGray,
  WhiteBaseColor,
  ChoiceBlue,
  ChoiceRed,
  TextBaseColor,
  BlackColor
} from "app/components/color";

import { CommentCard } from "./components/organisms/commentCard";
import { ChoiceList } from "./components/organisms/choiceList";
import { ReactComponent as SubmitIcon } from "./components/atoms/images/send.svg";

export const InstapollPage: React.FC = () => {
  return (
    <Container>
      <TimerComponent>
        <Timer>2:57</Timer>
      </TimerComponent>
      <CommentFeed>
        <CommentCard userName="Yuya_F" comment="レブロン調子いいね" flagColor={ChoiceBlue.hex}/>
        <CommentCard userName="Yuya_F" comment="レブロン調子いいね" flagColor={ChoiceRed.hex}/>
        <CommentCard userName="Yuya_F" comment="レブロン調子いいね" flagColor={ChoiceRed.hex}/>
        <CommentCard userName="Yuya_F" comment="レブロン調子いいね" flagColor={ChoiceBlue.hex}/>
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
}

const themeTitle = "次にポイントを決めるのは誰？"

const Container = styled.div`
  width: 100vw;
  height: calc(100vh - 75px);
  padding: 8px 28px;
  background-color: ${BackgroundMainColor.hex};
  user-select: none;
  position: relative;
`;

const TimerComponent = styled.div`
  width: 71px;
  height: 71px;
  margin 0px auto;
  border-radius: 50%;
  border: solid 3px ${ShadowGray.hex};
  position: relative;
  margin-bottom: 24px;
`;

const Timer = styled.span`
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit- transform: translateY(-50%) translateX(-50%);
  font-size: 16px;
  font-weight: 800;
  color: ${WhiteBaseColor.hex}
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

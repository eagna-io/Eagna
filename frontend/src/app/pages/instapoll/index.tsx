import React from "react";
import styled from "styled-components";

import {
  BackgroundMainColor,
  ShadowGray,
  WhiteBaseColor,
  ChoiceBlue,
  ChoiceRed
} from "app/components/color";

import { CommentCard } from "./components/organisms/commentCard";
import { ChoiceList } from "./components/organisms/choiceList";

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
      </PollCard>
    </Container>
  );
}

const themeTitle = "次にポイントを決めるのは誰？"

const Container = styled.div`
  width: 100vw;
  height: calc(100vh - 75px);
  padding: 8px 20px;
  background-color: ${BackgroundMainColor.hex};
  user-select: none;
`;

const TimerComponent = styled.div`
  width: 16vw;
  height: 16vw;
  margin 0px auto;
  border-radius: 50%;
  border: solid 3px ${ShadowGray.hex};
  position: relative;
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
  margin-top: 12px;
`;

const PollCard = styled.div`
  width: 100%;
  border-radius: 4px;
  margin-top: 24px;
  padding: 8px;
  background-color: ${WhiteBaseColor.hex}
`;

const Theme = styled.div`
  width: 100%;
  font-size: 16px;
`;

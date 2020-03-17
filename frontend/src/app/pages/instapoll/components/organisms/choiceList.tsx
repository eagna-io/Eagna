import React from "react";
import styled from "styled-components";

import {
  ChoiceBlue,
  ChoiceRed,
  WhiteBaseColor,
  VoteRateBackGround
} from "app/components/color";

import { ReactComponent as CorrectIcon } from "../atoms/images/correct.svg";

export const ChoiceList: React.FC = () => {
  return (
    <Container>
      <ChoiceContainer
        choiceTitle="Lebron"
        choiceColor={ChoiceBlue.hex}
        selected={true}
        voteRate={69}
        correct={true}
      />
      <ChoiceContainer
        choiceTitle="Kobe Bean Bryant"
        choiceColor={ChoiceRed.hex}
        selected={false}
        voteRate={30}
        correct={false}
      />
    </Container>
  );
}

const Container = styled.div`
  width: 100%;
  overflow: scroll;
`;

interface ChoiceContainerProps {
  choiceTitle: string;
  choiceColor: string;
  selected: boolean;
  voteRate: number;
  correct: boolean;
}

const ChoiceContainer: React.FC<ChoiceContainerProps> = ({ choiceTitle, choiceColor, selected, voteRate, correct }) => {
  return (
    <ChoiceOutcomeContainer>
      <OutcomeIcon correct={correct} /> 
      <Choice
        choiceTitle={choiceTitle}
        choiceColor={choiceColor}
        selected={selected}
        voteRate={voteRate}
      />
    </ChoiceOutcomeContainer>
  )
}

const ChoiceOutcomeContainer = styled.div`
  display: flex;
  position: relative;
  margin-bottom: 24px;
`;

const OutcomeIcon = styled(CorrectIcon)<{ correct: boolean }>`
  position: absolute;
  left: 0;
  width: 40px;
  height: 48px;
  display: ${ props => props.correct ? "block" : "none" };
`;

interface ChoiceProps {
  choiceTitle: string;
  choiceColor: string;
  selected: boolean;
  voteRate: number;
}

const Choice: React.FC<ChoiceProps> = ({ choiceTitle, choiceColor, selected, voteRate }) => {
  return (
    <StyledChoiceButton>
      <ChoiceButton choiceColor={choiceColor} selected={selected}></ChoiceButton>
      <VoteRate voteRate={voteRate}></VoteRate>
      <Choicetitle choiceColor={choiceColor} selected={selected}>{choiceTitle}</Choicetitle>
      <RateValue choiceColor={choiceColor} selected={selected}>{voteRate}%</RateValue>
    </StyledChoiceButton>
  )
}

const StyledChoiceButton = styled.div`
  position: relative;
  width: 200px;
  margin: 0px auto;
`;

const ChoiceButton = styled.button<{ choiceColor: string, selected: boolean }>`
  width: 200px;
  height: 48px;
  border-radius: 24px;
  padding: 4px 8px;
  border: solid 3px ${ props => props.choiceColor };
  background-color: ${ props => props.selected ? props.choiceColor : WhiteBaseColor.hex };
  font-size: 14px;
  font-weight: 600;
`;

const VoteRate = styled.div<{ voteRate: number }>`
  position: absolute;
  top: 3px;
  left: 3px;
  width: ${props => props.voteRate}%;
  height: 42px;
  border-radius: 24px;
  background-color: ${VoteRateBackGround.hexWithOpacity(0.5)};
`;

const Choicetitle = styled.div<{ choiceColor: string, selected: boolean }>`
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit- transform: translateY(-50%) translateX(-50%);
  width: 180px;
  text-align: left;
  color: ${ props => props.selected ? WhiteBaseColor.hex : props.choiceColor };
`;

const RateValue = styled.div<{ choiceColor: string, selected: boolean }>`
  position: absolute;
  top: 50%;
  right: 0;
  transform: translateY(-50%) translateX(-50%);
  -webkit- transform: translateY(-50%) translateX(-50%);
  color: ${ props => props.selected ? WhiteBaseColor.hex : props.choiceColor };
`;
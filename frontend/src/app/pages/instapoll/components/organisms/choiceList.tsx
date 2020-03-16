import React from "react";
import styled from "styled-components";

import {
  ChoiceBlue,
  ChoiceRed,
  WhiteBaseColor
} from "app/components/color";

interface Props {}

export const ChoiceList: React.FC<Props> = () => {
  return (
    <Container>
      <Choice choiceTitle="Lebron" choiceColor={ChoiceBlue.hex} selected={true} />
      <Choice choiceTitle="Kobe Bean Bryant" choiceColor={ChoiceRed.hex} selected={false} />
    </Container>
  );
}

const Container = styled.div`
  width: 100%;
  padding: 10px;
  overflow: scroll;
`;

interface ChoiceProps {
  choiceTitle: string;
  choiceColor: string;
  selected: boolean;
}

const Choice: React.FC<ChoiceProps> = ({ choiceTitle, choiceColor, selected }) => {
  return (
    <StyledChoiceButton>
      <ChoiceButton choiceColor={choiceColor} selected={selected}>{choiceTitle}</ChoiceButton>
    </StyledChoiceButton>
  )
}

const StyledChoiceButton = styled.div`
  width: 180px;
  margin: 8px auto;
`;

const ChoiceButton = styled.button<{ choiceColor: string, selected: boolean }>`
  width: 180px;
  height: 40px;
  border-radius: 24px;
  padding: 4px 8px;
  border: solid 3px ${ props => props.choiceColor };
  color: ${ props => props.selected ? WhiteBaseColor.hex : props.choiceColor };
  background-color: ${ props => props.selected ? props.choiceColor : WhiteBaseColor.hex };
  font-size: 14px;
  font-weight: 600;
`;

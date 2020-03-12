import React from "react";
import styled from "styled-components";
import { RedDisagreeColor, GreenAgreeColor } from "app/components/color";

import { PressButton } from "app/components/atoms/press-button";

import { Outcome } from "../../reducer";
import { ReactComponent as UpVoteIcon } from "../atoms/images/vote-up.svg";
import { ReactComponent as DownVoteIcon } from "../atoms/images/vote-down.svg";

interface Props {
  onVote: (outcome: Outcome) => void;
}

export const VoteButtons: React.FC<Props> = ({ onVote }) => {
  return (
    <Container>
      <Button
        color={RedDisagreeColor.hex}
        onClick={() => onVote("unrealize")}
        outcome="unrealize"
      />
      <Button
        color={GreenAgreeColor.hex}
        onClick={() => onVote("realize")}
        outcome="realize"
      />
    </Container>
  );
};

const Container = styled.div`
  display: flex;
  justify-content: space-between;
  padding: 0px 20px;
`;

interface ButtonProps {
  color: string;
  onClick: () => void;
  outcome: Outcome;
}

const Button: React.FC<ButtonProps> = ({ color, onClick, outcome }) => {
  return (
    <StyledPressButton color={color} onPress={onClick} threshold={1000}>
      <ButtonContainer>
        {outcome === "unrealize" ? (
          <StyledDownVoteIcon />
        ) : (
          <StyledUpVoteIcon />
        )}
        <PredictionValue color={color}>
          83%{outcome === "unrealize" ? "未満" : "以上"}
        </PredictionValue>
      </ButtonContainer>
    </StyledPressButton>
  );
};

const StyledPressButton = styled(PressButton)<{ color: string }>`
  width: calc(50% - 8px);
  height: 50px;
  border: 4px solid ${props => props.color};
  border-radius: 8px;
  line-height: 50px;
  font-size: 36px;
  text-align: center;
  transition: transform 0.1s ease;

  &:active {
    transform: scale(0.95);
  }
`;

const StyledUpVoteIcon = styled(UpVoteIcon)`
  width: 32px;
  height: 42px;
`;

const StyledDownVoteIcon = styled(DownVoteIcon)`
  width: 32px;
  height: 42px;
`;

const ButtonContainer = styled.div`
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
`;

const PredictionValue = styled.div<{ color: string }>`
  position: absolute;
  right: 0;
  width: 40px;
  line-height: 12px;
  padding-right: 12px;
  font-size: 12px;
  color: ${props => props.color};
`;

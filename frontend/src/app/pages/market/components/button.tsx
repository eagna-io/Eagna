import React from "react";
import styled from "styled-components";

import { PressButton } from "app/components/atoms/press-button";

interface Props {
  onVote: (outcome: string) => void;
}

const Buttons: React.FC<Props> = ({ onVote }) => {
  return (
    <Container>
      <Button color="#F74C61" onClick={() => onVote("lose")} text="-" />
      <Button color="#39CCBE" onClick={() => onVote("win")} text="+" />
    </Container>
  );
};

export default Buttons;

const Container = styled.div`
  display: flex;
  justify-content: space-between;
  margin-top: 15px;
`;

interface ButtonProps {
  color: string;
  onClick: () => void;
  text: string;
}

const Button: React.FC<ButtonProps> = ({ color, onClick, text }) => {
  return (
    <StyledPressButton color={color} onPress={onClick} threshold={1000}>
      {text}
    </StyledPressButton>
  );
};

const StyledPressButton = styled(PressButton)<{ color: string }>`
  width: calc(50% - 8px);
  height: 126px;
  background-color: ${props => props.color};
  line-height: 126px;
  font-size: 36px;
  text-align: center;
  transition: transform 0.1s ease;

  &:active {
    transform: scale(0.95);
  }
`;

import React from "react";
import styled from "styled-components";

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
    <ButtonContainer color={color} onClick={onClick}>
      {text}
    </ButtonContainer>
  );
};

const ButtonContainer = styled.div<{ color: string }>`
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

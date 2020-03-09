import React from "react";
import styled from "styled-components";
import { RedDisagreeColor, GreenAgreeColor} from "app/components/color";

import { PressButton } from "app/components/atoms/press-button";

import UpVote from "../atoms/images/vote-up.svg";
import DownVote from "../atoms/images/vote-down.svg";

interface Props {
  onVote: (outcome: string) => void;
}

const Buttons: React.FC<Props> = ({ onVote }) => {
  return (
    <Container>
      <Button color={RedDisagreeColor.hex} onClick={() => onVote("lose")} chkVote={false} /> 
      <Button color={GreenAgreeColor.hex} onClick={() => onVote("win")} chkVote={true} />
    </Container>
  );
};

export default Buttons;

const Container = styled.div`
  display: flex;
  justify-content: space-between;
  padding: 0px 20px;
`;

interface ButtonProps {
  color: string;
  onClick: () => void;
  chkVote: boolean;
}

const Button: React.FC<ButtonProps> = ({ color, onClick, chkVote }) => {
  return (
    <StyledPressButton color={color} onPress={onClick} threshold={1000}>
      <ButtonContainer>
        {chkVote ? <Img src={UpVote} alt="UpVote"/> : <Img src={DownVote} alt="DownVote"/>}
        <PredictionValue color={color}>83%{chkVote ? "以上" : "未満"}</PredictionValue>
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

const Img = styled.img`
  width: 32px;
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

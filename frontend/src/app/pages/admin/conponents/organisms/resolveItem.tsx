import React from "react";
import styled from "styled-components";

import {
  AdminMainColor,
  AdminInputBorderColor
} from "app/components/color";

interface Props {
  choiceItem: string;
  confirmTitle: string;
}

export const ResolveItem: React.FC<Props> = ({choiceItem, confirmTitle}) => {

  const handleResolve = () => {
    window.confirm("「" + confirmTitle + "」を「" + choiceItem + "」でResoveします、よろしいですか？");
  }

  return (
    <Container>
      <ChoiceItem>{choiceItem}</ChoiceItem>
      <ResolveButton onClick={handleResolve}>Resolve</ResolveButton>
    </Container>
  );
}

const Container = styled.div`
  margin-bottom: 89px;
  color: ${AdminMainColor.hex};
  display: flex;
  justify-content: space-between;
  align-item: middle;
`;

const ChoiceItem = styled.div`
  font-size: 18px;
  line-height: 45px;
  font-weight: 500;
  color: ${AdminMainColor.hex};
`;

const ResolveButton = styled.div`
  width: 120px;
  height: 45px;
  padding: 9px 26px;
  font-size: 18px;
  font-weight: 500;
  border-radius: 8px;
  color: ${AdminInputBorderColor.hex};
  border: solid 1px ${AdminInputBorderColor.hex};
`;

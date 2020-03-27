import React from "react";
import styled from "styled-components";

import {
  NavBarBackgroundColor,
  AdminResolvePollButtonBorderColor
} from "app/components/color";

interface Props {
  choiceItem: string;
}

export const ResolveItem: React.FC<Props> = ({choiceItem}) => {
  return (
    <Container>
      <ChoiceItem>{choiceItem}</ChoiceItem>
      <ResolveButton>Resolve</ResolveButton>
    </Container>
  );
}

const Container = styled.div`
  margin-bottom: 89px;
  color: ${NavBarBackgroundColor.hex};
  display: flex;
  justify-content: space-between;
  align-item: middle;
`;

const ChoiceItem = styled.div`
  font-size: 18px;
  line-height: 45px;
  font-weight: 500;
  color: ${NavBarBackgroundColor.hex};
`;

const ResolveButton = styled.div`
  width: 120px;
  height: 45px;
  padding: 9px 26px;
  font-size: 18px;
  font-weight: 500;
  border-radius: 8px;
  color: ${AdminResolvePollButtonBorderColor.hex};
  border: solid 1px ${AdminResolvePollButtonBorderColor.hex};
`;

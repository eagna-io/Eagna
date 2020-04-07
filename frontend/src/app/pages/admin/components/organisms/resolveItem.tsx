import React from "react";
import styled from "styled-components";

import { AdminMainColor } from "app/components/color";
import * as pollApi from "infra/http/poll";

interface Props {
  choiceItem: string;
  pollTitle: string;
}

export const ResolveItem: React.FC<{
  choiceItem: string;
  pollTitle: string;
}> = ({ choiceItem, pollTitle }) => {
  const handleResolve = () => {
    const confirmed = window.confirm(
      `「 ${pollTitle} 」を「${choiceItem}」でResoveします、よろしいですか？`
    );
    if (confirmed) {
      pollApi.resolve(choiceItem).then(res => alert(JSON.stringify(res)));
    }
  };

  return (
    <Container>
      <ChoiceItem>{choiceItem}</ChoiceItem>
      <ResolveButton onClick={handleResolve}>Resolve</ResolveButton>
    </Container>
  );
};

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

const ResolveButton = styled.button`
  width: 120px;
  height: 45px;
  padding: 9px 26px;
  font-size: 18px;
  font-weight: 500;
  border-radius: 8px;
  color: ${AdminMainColor.hex};
  border: solid 1px ${AdminMainColor.hex};
`;

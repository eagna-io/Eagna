import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  TextBaseColor,
  ChoiceBlue
} from "app/components/color";

interface Props {
  userName: string;
  comment: string;
}

export const CommentCard: React.FC<Props> = ({ userName, comment }) => {
  return (
    <Container>
      <Contents>
        <UserName>{userName}</UserName>
        <Comment>{comment}</Comment>
      </Contents>
      <ChoiceFlag></ChoiceFlag>
    </Container>
  );
}

const Container = styled.div`
  width: 100%;
  margin 4px 0px;
  border-radius: 4px;
  background-color: ${WhiteBaseColor.hex}
  display: flex;
  justify-content: space-between;
`;

const Contents = styled.div`
  padding: 4px 8px;
  color: ${TextBaseColor.hex}
`;

const UserName = styled.div`
  font-size: 6px;
`;

const Comment = styled.div`
  font-size: 12px;
  font-weight: 500;
`;

const ChoiceFlag = styled.div`
  width: 20px;
  border-radius: 0px 4px 4px 0px;
  background: linear-gradient(to bottom right, rgba(255,255,255,0) 50%, ${ChoiceBlue.hex} 50.5%) no-repeat top left/100% 100%;
`;

import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  TextBaseColor,
  BlackColor
} from "app/components/color";
import { Comment as CommentModel } from "../../models";

interface Props {
  comment: CommentModel;
}

export const CommentCard: React.FC<Props> = ({ comment }) => {
  return (
    <Container>
      <Contents>
        <UserName>{comment.account}</UserName>
        <Comment>{comment.comment}</Comment>
      </Contents>
      <ChoiceFlag flagColor={comment.color}></ChoiceFlag>
    </Container>
  );
};

const Container = styled.div`
  width: 100%;
  height: 36px;
  margin-bottom: 16px;
  border-radius: 4px;
  background-color: ${WhiteBaseColor.hex}
  display: flex;
  justify-content: space-between;
  box-shadow: 0 2px 4px 0 ${BlackColor.rgba(0.5)};
`;

const Contents = styled.div`
  padding: 2px 6px;
  color: ${TextBaseColor.hex};
`;

const UserName = styled.div`
  font-size: 8px;
`;

const Comment = styled.div`
  font-size: 12px;
  font-weight: 500;
`;

const ChoiceFlag = styled.div<{ flagColor: string }>`
  width: 25px;
  border-radius: 0px 4px 4px 0px;
  background: linear-gradient(
      to bottom right,
      rgba(255, 255, 255, 0) 50%,
      ${props => props.flagColor} 50.5%
    )
    no-repeat top left/100% 100%;
`;

import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  Gallery
} from "app/components/color";
import { Comment as CommentModel } from "model/poll";

interface Props {
  comment: CommentModel;
}

export const CommentCard: React.FC<Props> = ({ comment }) => {
  return (
    <Container>
      <Account>
        <ChoiceFlag flagColor={comment.color}></ChoiceFlag>
        <UserName>{comment.account}</UserName>
      </Account>
      <Comment>{comment.comment}</Comment>
    </Container>
  );
};

const Container = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: middle;
  width: 100%;
  margin-bottom: 16px;
  color: ${WhiteBaseColor.hex};
  &:last-child {
    margin-bottom: 0;
  }
`;

const Account = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: middle;
  height: 16px;
  margin-right: 4px;
  font-weight: 400;
`;

const ChoiceFlag = styled.div<{ flagColor: string }>`
  width: 16px;
  height: 16px;
  border-radius: 50%;
  margin-right: 8px;
  background-color: ${props => props.flagColor}
`;

const UserName = styled.div`
  font-size: 8px;
  line-height: 16px;
`;

const Comment = styled.div`
  font-size: 12px;
  line-height: 16px;
  color: ${Gallery.hex};
`;

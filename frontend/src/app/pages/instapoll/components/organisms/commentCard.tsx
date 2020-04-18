import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  Gallery
} from "app/components/color";
import { Comment as CommentModel } from "model/poll";

interface Props {
  comment: CommentModel & { color: string };
}

export const CommentCard: React.FC<Props> = ({ comment }) => {
  return (
    <Container>
      <FlagBlock>
        <ChoiceFlag flagColor={comment.color}></ChoiceFlag>
      </FlagBlock>  
      <UserName>{comment.account_name}</UserName>
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

// Memo:CommentのheightにFlagの円（radius）が影響されるためFlagBlockで切り出す対応
const FlagBlock = styled.div`
  height: 16px;
`;

const ChoiceFlag = styled.div<{ flagColor: string }>`
  width: 16px;
  height: 16px;
  border-radius: 50%;
  margin-right: 8px;
  background-color: ${props => props.flagColor}
`;

const UserName = styled.div`
  max-width: 80px;
  margin-right: 4px;
  font-size: 8px;
  line-height: 16px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

const Comment = styled.div`
  max-width: 220px;
  font-size: 12px;
  line-height: 16px;
  color: ${Gallery.hex};
`;

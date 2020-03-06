import React from "react";
import styled from "styled-components";

import { Record } from "model/chart";

interface Props {
  records: Record[];
}

const Feed: React.FC<Props> = ({ records }) => {
  return (
    <Container>
      {[...records].reverse().map(record => (
        <Item key={record.time} record={record} />
      ))}
    </Container>
  );
};

export default Feed;

const Item: React.FC<{ record: Record }> = ({ record }) => {
  const mine = record.user === "たかはしあつき";
  const color = record.outcome === "win" ? "#39CCBE" : "#F74C61";

  return (
    <ItemContainer unselectable="on">
      <Name mine={mine}>{record.user}</Name>
      <Outcome color={color}>{record.outcome}</Outcome>
      と予想しました
    </ItemContainer>
  );
};

const Container = styled.div`
  width: 100%;
  height: 180px;
  overflow: scroll;
  padding: 10px;
  margin-top: 70px;
  background-color: rgba(36, 36, 35, 0);
`;

const ItemContainer = styled.div`
  width: 50%;
  padding: 2px 14px;
  border-radius: 9px;
  background-color: rgba(70, 70, 70, 0.5);
  margin-bottom: 5px;
  font-size: 12px;
  font-weight: 200;
  color: white;
  user-select: none;
`;

const Name = styled.div<{ mine: boolean }>`
  color: ${props => (props.mine ? "#F8E71C" : "#ffffff")};
  font-size: 8px;
`;

const Outcome = styled.span<{ color: string }>`
  color: ${props => props.color};
  font-size: 12px;
`;

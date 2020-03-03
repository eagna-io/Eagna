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
    <ItemContainer>
      <Name mine={mine}>{record.user}</Name>さんが
      <Outcome color={color}>{record.outcome}</Outcome>
      と予想しました
    </ItemContainer>
  );
};

const Container = styled.div`
  width: 100%;
  height: 155px;
  overflow: scroll;
  padding: 10px;
  background-color: #242423;
`;

const ItemContainer = styled.div`
  width: 100%;
  padding: 9px 14px;
  margin-bottom: 5px;
  background-color: #333333;
  font-size: 12px;
  font-weight: 200;
  color: white;
`;

const Name = styled.span<{ mine: boolean }>`
  color: ${props => (props.mine ? "#F8E71C" : "#BB86FC")};
`;

const Outcome = styled.span<{ color: string }>`
  color: ${props => props.color};
`;

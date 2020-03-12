import React from "react";
import styled from "styled-components";
import { 
  RankingColor,
  TextBaseColor,
  GreenAgreeColor,
  RedDisagreeColor,
  ItemContainerBgColor
} from "app/components/color";

import { FeedItem, Outcome } from "../../reducer";

interface Props {
  records: FeedItem[];
}

const Feed: React.FC<Props> = ({ records }) => {
  return (
    <Container>
      {[...records].reverse().map(record => (
        <Item key={record.id} record={record} />
      ))}
    </Container>
  );
};

export default Feed;

const Item: React.FC<{ record: FeedItem }> = ({ record }) => {
  const mine = record.accountName === "たかはしあつき";
  const displayOutcome = record.outcome === "realize" ? "実現する" : "実現しない";

  return (
    <ItemContainer mine={mine} unselectable="on">
      <Name mine={mine}>{record.accountName}</Name>
      <OutcomeComponent outcome={record.outcome}>{displayOutcome}</OutcomeComponent>
      と予想しました
    </ItemContainer>
  );
};

const Container = styled.div`
  width: 100%;
  height: 24vh;
  overflow: scroll;
  padding: 10px;
  margin-top: 20px;
`;

const ItemContainer = styled.div<{ mine: boolean }>`
  width: 50%;
  padding: 2px 14px;
  border-radius: 9px;
  background-color: ${props => props.mine ? RankingColor.hexWithOpacity(0.5) : ItemContainerBgColor.hexWithOpacity(0.5)}};
  margin-bottom: 5px;
  font-size: 12px;
  font-weight: 200;
  color: ${TextBaseColor.hex};
  user-select: none;
`;

const Name = styled.div<{ mine: boolean }>`
  color: ${props => props.mine ? RankingColor.hex : TextBaseColor.hex};
  font-size: 6px;
`;

const OutcomeComponent = styled.span<{ outcome: Outcome }>`
  color: ${props => props.outcome === "realize" ? GreenAgreeColor.hex : RedDisagreeColor.hex};
  font-size: 12px;
`;

import React from "react";
import styled from "styled-components";
import { 
  WhiteBaseColor,
  RankingColor,
  GreenAgreeColor,
  RedDisagreeColor,
  ItemContainerBgColor
} from "app/components/color";

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
  const win = record.outcome === "win";

  return (
    <ItemContainer unselectable="on">
      <Name mine={mine}>{record.user}</Name>
      <Outcome win={win}>{record.outcome}</Outcome>
      と予想しました
    </ItemContainer>
  );
};

const Container = styled.div`
  width: 100%;
  height: 172px;
  overflow: scroll;
  padding: 10px;
  margin-top: 30px;
`;

const ItemContainer = styled.div`
  width: 50%;
  padding: 2px 14px;
  border-radius: 9px;
  background-color: ${ItemContainerBgColor.hexWithOpacity(0.5)};
  margin-bottom: 5px;
  font-size: 12px;
  font-weight: 200;
  color: #AEAEAE;
  user-select: none;
`;

const Name = styled.div<{ mine: boolean }>`
  color: ${props => props.mine ? RankingColor.hex : WhiteBaseColor.hex};
  font-size: 6px;
`;

const Outcome = styled.span<{ win: boolean }>`
  color: ${props => props.win ? GreenAgreeColor.hex : RedDisagreeColor.hex};
  font-size: 12px;
`;

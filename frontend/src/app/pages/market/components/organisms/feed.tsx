import React from "react";
import styled from "styled-components";
import { 
  Color,
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
  const myColor = mine ? RankingColor.hex : WhiteBaseColor.hex;
  const color = record.outcome === "win" ? GreenAgreeColor.hex : RedDisagreeColor.hex;

  return (
    <ItemContainer unselectable="on" itemContainerBgColor={ItemContainerBgColor}>
      <Name mine={mine} myColor={myColor}>{record.user}</Name>
      <Outcome color={color}>{record.outcome}</Outcome>
      と予想しました
    </ItemContainer>
  );
};

const Container = styled.div`
  width: 100%;
  height: 172px;
  overflow: scroll;
  padding: 10px;
  margin-top: 70px;
`;

const ItemContainer = styled("div")<{ itemContainerBgColor: Color }>`
  width: 50%;
  padding: 2px 14px;
  border-radius: 9px;
  background-color: ${props => props.itemContainerBgColor.hexWithOpacity(0.5)};
  margin-bottom: 5px;
  font-size: 12px;
  font-weight: 200;
  color: #AEAEAE;
  user-select: none;
`;

const Name = styled("div")<{ mine: boolean,  myColor: string }>`
  color: ${props => props.myColor};
  font-size: 6px;
`;

const Outcome = styled.span<{ color: string }>`
  color: ${props => props.color};
  font-size: 12px;
`;

import React from "react";
import styled from "styled-components";

interface Props {
  marketTitle: string;
  ranking: number;
  paticipantsNum: number;
}

const Contents: React.FC<Props> = ({ marketTitle,ranking, paticipantsNum }) => {

  return (
    <Container>
      <MarketTitle>{marketTitle}</MarketTitle>
      <Ranking>
        予測ランキング
        <RankingValue>
          <RankNum>{ranking}</RankNum>位｜{paticipantsNum}人中
        </RankingValue>
      </Ranking>
    </Container>
  );
};

export default Contents;

const Container = styled.div`
  position: relative;
  background-color: #242423;
  padding: 0 20px 20px;
`;

const MarketTitle = styled.div`
  font-size: 18px;
  color: #BB86FC;
  font-weight: 300;
`;

const Ranking = styled.div`
  margin-top: 8px;
  color: #AEAEAE;
  font-size: 14px;
  font-weight: 600;
`;

const RankingValue = styled.div`
  margin-left: 8px;
  letter-spacing: 1px;
`;

const RankNum = styled.span`
  font-size: 24px;
  color: #FAD160;
  font-weight: 800;
  margin-right: 4px;
`;
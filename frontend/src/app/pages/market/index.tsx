import React from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";
import { Color, BackgroundMainColor, PurpleColor, TextBaseColor, RankingColor } from "app/components/color";

import { RootState } from "app/redux";
import { actions } from "app/redux/chart";
import { now } from "model/time";

import Header from "./components/organisms/header";
import ChartContainer from "./components/organisms/chartContainer";
import Feed from "./components/organisms/feed";

const Page: React.FC = () => {

  const records = useSelector((state: RootState) => state.chart.records);
  const dispatch = useDispatch();

  React.useEffect(() => {
    let handler = setInterval(() => {
      const user = botNames[Math.floor(Math.random() * 1000) % botNames.length];
      const time = now();
      const outcome = Math.random() >= 0.5 ? "win" : "lose";
      dispatch(actions.vote({ outcome, time, user }));
    }, 200);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  return (
    <Container bgcolor={BackgroundMainColor}>
      <ChartContainer />
      <SubContainer bgcolor={BackgroundMainColor}>
        <Header userName="Yuya_F" />
        <MarketTitle purpleText={PurpleColor}>{marketTitle}</MarketTitle>
        <Ranking textBaseColor={TextBaseColor}>
          予測ランキング
          <RankingValue>
            <RankNum rankingColor={RankingColor}>{ranking}</RankNum>位｜{paticipantsNum}人中
          </RankingValue>
        </Ranking>
        <Feed records={records} />
      </SubContainer>
    </Container>
  );
};

export default Page;

const marketTitle = 'RAGE Shadowverse 2020 Spring'
const ranking = 2
const paticipantsNum = 358

const botNames = [
  "ふるさわゆうや",
  "ふなはしこうき",
  "ドナルドトランプ",
  "きしべろはん"
];


const Container = styled("div")<{ bgcolor: Color }>`
  width: 100vw;
  height: 100vh;
  background-color:  ${props => props.bgcolor.hex};
`;

const SubContainer = styled("div")<{ bgcolor: Color }>`
  position: relative;
  width: 100vw;
  background-color: rgba(${props => props.bgcolor.hex}, 0.5);
  padding: 20px;
`;

const MarketTitle = styled("div")<{ purpleText: Color }>`
  font-size: 18px;
  color: ${props => props.purpleText.hex};
  font-weight: 300;
`;

const Ranking = styled("div")<{ textBaseColor: Color }>`
  margin-top: 8px;
  color: ${props => props.textBaseColor.hex};
  font-size: 14px;
  font-weight: 600;
`;

const RankingValue = styled.div`
  margin-left: 8px;
  letter-spacing: 1px;
`;

const RankNum = styled("span")<{ rankingColor: Color }>`
  font-size: 24px;
  color: ${props => props.rankingColor.hex};
  font-weight: 800;
  margin-right: 4px;
`;

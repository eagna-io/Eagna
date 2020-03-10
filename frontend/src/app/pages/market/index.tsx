import React from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";
import {
  BackgroundMainColor,
  PurpleColor,
  TextBaseColor,
  RankingColor
} from "app/components/color";

import { RootState } from "app/redux";
import { actions, Data } from "app/redux/chart";
import { now } from "model/time";

import * as ws from "infra/ws";
import { getMarketInfo } from "infra/rpc/get_market_info";
import { vote } from "infra/rpc/vote";

import { reducer, initialState } from "./reducer";
import Header from "./components/organisms/header";
import ChartContainer from "./components/organisms/chartContainer";
import Feed from "./components/organisms/feed";
import { VoteButtons } from "./components/organisms/voteButton";

interface Props {
  marketId: string;
}

export const MarketPage: React.FC<Props> = ({ marketId }) => {
  const [datasets, records] = useSelector((state: RootState) => [
    state.chart.datasets,
    state.chart.records
  ]);
  const [state, dispatch] = React.useReducer(reducer, initialState);

  // 対象のマーケットページを初めて開いた時にWebSocketコネクションを貼る
  // FeedMsgを受け取るたびにFeedに書き込む
  React.useEffect(() => {
    let unmounted = false;

    dispatch({ type: "clear" });

    (async () => {
      // まずマーケットの情報を取得
      const { title, outcomes } = await getMarketInfo({ marketId });
      if (unmounted) {
        return;
      }
      dispatch({
        type: "setMarketInfo",
        title,
        outcomes
      });

      // WebSocketコネクションの確立
      // TODO: close処理
      ws.open({
        marketId,
        onFeedMsg: msg => {
          dispatch({
            type: "addFeedItem",
            outcomeId: msg.outcomeId,
            userName: msg.accountName
          });
        }
      });
    })();

    return () => {
      unmounted = true;
    };
  }, [marketId]);

  const publicPred = getPublicPrediction(datasets.win);

  return (
    <Container>
      <ChartContainer />
      <SubContainer>
        <Header userName="Yuya_F" />
        <MarketTitle>{marketTitle}</MarketTitle>
        <Ranking>
          予測ランキング
          <RankingValue>
            <RankNum>{ranking}</RankNum>位｜{paticipantsNum}人中
          </RankingValue>
        </Ranking>
        <Feed records={records} />
      </SubContainer>
      <Guide>
        <PredictionTheme>{predictionTheme}</PredictionTheme>
        <PublicPrediction>
          みんなの予想
          <PredictionValue>
            {publicPred}
            <Sm>%</Sm>
          </PredictionValue>
        </PublicPrediction>
      </Guide>
      <VoteButtons
        onVote={outcome =>
          // TODO
          undefined
        }
      />
    </Container>
  );
};

const getPublicPrediction = (data: Data[]): string => {
  if (data.length === 0) {
    return "-";
  } else {
    return Math.floor(data[data.length - 1][1] / 10).toString();
  }
};

const marketTitle = "RAGE Shadowverse 2020 Spring";
const ranking = 2;
const paticipantsNum = 358;
const predictionTheme = "GRAND FINALS Shimon/REVが優勝する確率を予想せよ";

const botNames = [
  "ふるさわゆうや",
  "ふなはしこうき",
  "ドナルドトランプ",
  "きしべろはん"
];

const Container = styled.div`
  width: 100vw;
  height: calc(100vh - 75px);
  background-color: ${BackgroundMainColor.hex};
  user-select: none;
`;

const SubContainer = styled.div`
  position: relative;
  width: 100vw;
  padding: 20px;
  height: 60vh;
`;

const MarketTitle = styled.div`
  font-size: 18px;
  color: ${PurpleColor.hex};
  font-weight: 300;
`;

const Ranking = styled.div`
  margin-top: 8px;
  color: ${TextBaseColor.hex};
  font-size: 14px;
  font-weight: 600;
`;

const RankingValue = styled.div`
  margin-left: 8px;
  letter-spacing: 1px;
`;

const RankNum = styled.span`
  font-size: 24px;
  color: ${RankingColor.hex};
  font-weight: 800;
  margin-right: 4px;
`;

const Guide = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0px 20px 0px 20px;
`;

const PredictionTheme = styled.div`
  color: ${TextBaseColor.hex}
  font-size: 14px;
  whiteSpace: 'pre-line';
  width: 70%;
`;

const PublicPrediction = styled.div`
  color: ${TextBaseColor.hex}
  text-align: center;
  font-size: 14px;
  font-weight: 600;
  width: 30%;
`;

const PredictionValue = styled.div`
  font-size: 56px;
  line-height: 56px;
  font-weight: 800;
`;

const Sm = styled.span`
  font-size: 20px;
  font-weight: 800;
`;

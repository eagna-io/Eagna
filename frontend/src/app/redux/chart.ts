import { createSlice, PayloadAction, CaseReducer } from "@reduxjs/toolkit";
import moment from "moment";

import {
  Distribution,
  Record,
  create,
  increment,
  lmsrPrice
} from "model/chart";
import { DateTime, now } from "model/time";
import { Map, forEach, entries } from "model/map";

/*
 * =========
 * State
 * =========
 */
export type State = {
  snapshot: ChartSnapshot;
  datasets: Map<Data[]>;
  userPaid: number;
  userAsset: Map<number>;
  userScore: number;
};

export type ChartSnapshot = {
  distribution: Distribution;
  time: DateTime;
};

export type Data = [DateTime, number];

const MAX_HISTORY_RECORDS: number = 20;
const MAX_HISTORY_DURATION: number = 60;

/*
 * =============
 * Reducers
 * =============
 */
type VotePayload = {
  outcome: string;
  time: DateTime;
  user: string;
};

const vote: CaseReducer<State, PayloadAction<VotePayload>> = (
  state,
  action
) => {
  const { outcome, time, user } = action.payload;

  // Chart snapshotの更新
  // 次の分布の計算
  const nextDistribution = increment(state.snapshot.distribution, outcome);

  // 新しい取引履歴の作成
  const record = {
    outcome,
    price: nextDistribution.lmsrCost - state.snapshot.distribution.lmsrCost,
    time,
    user
  };

  /*
   * stateの更新
   */
  // snapshotの更新
  state.snapshot = {
    distribution: nextDistribution,
    time
  };

  // datasetの更新
  forEach(state.datasets, (outcome, datas) => {
    const newData = [time, lmsrPrice(nextDistribution, outcome)] as Data;
    datas.push(newData);

    // 最大保存件数を超えているか
    if (datas.length > MAX_HISTORY_RECORDS) {
      const oldestDate = moment(datas[0][0]).unix();
      const now = moment(time).unix();
      // 最大保存期間を過ぎているか
      if (oldestDate < now - MAX_HISTORY_DURATION) {
        datas.shift();
      }
    }
  });

  // userの更新
  if (user === "たかはしあつき") {
    state.userPaid += record.price;
    state.userAsset[outcome] += 1;
  }

  // userScoreの更新
  if (state.userPaid > 0) {
    const marketCap = entries(state.userAsset).reduce((acc, [outcome, q]) => {
      const datas = state.datasets[outcome];
      const price = datas.length === 0 ? 0 : datas[datas.length - 1][1];
      return acc + q * price;
    }, 0);
    state.userScore = marketCap / state.userPaid;
  }
};

/*
 * =========
 * Slice
 * =========
 */
export const { actions, reducer } = createSlice({
  name: "chart",
  initialState: {
    snapshot: {
      distribution: create({ win: 0, lose: 0 }),
      time: now()
    },
    datasets: { win: [], lose: [] },
    userPaid: 0,
    userAsset: { win: 0, lose: 0 },
    userScore: 0
  } as State,
  reducers: {
    vote
  }
});

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
import { Map, forEach } from "model/map";

/*
 * =========
 * State
 * =========
 */
export type State = {
  snapshot: ChartSnapshot;
  recentHistory: Record[];
  datasets: Map<Data[]>;
};

export type ChartSnapshot = {
  distribution: Distribution;
  time: DateTime;
};

export type Data = [DateTime, number];

const MAX_HISTORY_RECORDS: number = 100;
const MAX_HISTORY_DURATION: number = 60;

/*
 * =============
 * Reducers
 * =============
 */
type VotePayload = {
  outcome: string;
  time: DateTime;
};

const vote: CaseReducer<State, PayloadAction<VotePayload>> = (
  state,
  action
) => {
  const { outcome, time } = action.payload;

  // Chart snapshotの更新
  const nextDistribution = increment(state.snapshot.distribution, outcome);
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

  // オーダー履歴の更新
  const record = {
    outcome,
    price: nextDistribution.lmsrCost - state.snapshot.distribution.lmsrCost,
    time
  };
  state.recentHistory.push(record);
  if (state.recentHistory.length > MAX_HISTORY_RECORDS) {
    state.recentHistory.shift();
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
    recentHistory: []
  } as State,
  reducers: {
    vote
  }
});

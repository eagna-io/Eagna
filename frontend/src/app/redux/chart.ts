import { createSlice, PayloadAction, CaseReducer } from "@reduxjs/toolkit";

import { Distribution, Record, create, increment } from "model/chart";
import { Array, push, empty } from "model/array";
import { DateTime, now } from "model/time";

/*
 * =========
 * State
 * =========
 */
type State = {
  snapshot: {
    distribution: Distribution;
    time: DateTime;
  };
  recentHistory: Record[];
};

const MAX_HISTORY_RECORDS: number = 100;

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
    recentHistory: []
  } as State,
  reducers: {
    vote
  }
});

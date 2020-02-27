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
  history: Record[];
};

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

  // アウトカムの新しい分布
  const nextDistribution = increment(state.snapshot.distribution, outcome);

  // 今回のオーダーを表すレコード
  const record = {
    outcome,
    price: nextDistribution.lmsrCost - state.snapshot.distribution.lmsrCost,
    time
  };

  state.snapshot = {
    distribution: nextDistribution,
    time
  };
  state.history.push(record);
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
    history: []
  } as State,
  reducers: {
    vote
  }
});

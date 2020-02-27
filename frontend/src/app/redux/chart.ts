import { createSlice, PayloadAction, CaseReducer } from "@reduxjs/toolkit";
import { List, Map } from "immutable";
import { Moment } from "moment";

import { Distribution, Record } from "model/chart";

/*
 * =========
 * State
 * =========
 */
type State = {
  distribution: Distribution;
  history: List<Record>;
};

/*
 * =============
 * Reducers
 * =============
 */
type VotePayload = {
  outcome: string;
  time: Moment;
};

const vote: CaseReducer<State, PayloadAction<VotePayload>> = (
  state,
  action
) => {
  const { outcome, time } = action.payload;

  // アウトカムの新しい分布
  const nextDistribution = state.distribution.increment(outcome);

  // 今回のオーダーを表すレコード
  const record = {
    outcome,
    price: nextDistribution.lmsrCost - state.distribution.lmsrCost,
    time,
    prevDistribution: state.distribution as Distribution,
    nextDistribution
  };

  return {
    distribution: nextDistribution,
    history: state.history.push(record)
  };
};

/*
 * =========
 * Slice
 * =========
 */
export const { actions, reducer } = createSlice({
  name: "chart",
  initialState: {
    distribution: Distribution.initialize(List(["win", "lose"])),
    history: List()
  } as State,
  reducers: {
    vote
  }
});

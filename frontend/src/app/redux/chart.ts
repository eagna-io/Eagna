import { BinaryChart, Record } from "model/chart";
import {
  configureStore,
  createSlice,
  PayloadAction,
  CaseReducer
} from "@reduxjs/toolkit";

/*
 * =============
 * Reducers
 * =============
 */
const addWinRecord: CaseReducer<BinaryChart, PayloadAction<Record>> = (
  state,
  action
) => state.addWin(action.payload);

const addLoseRecord: CaseReducer<BinaryChart, PayloadAction<Record>> = (
  state,
  action
) => state.addLose(action.payload);

/*
 * =========
 * Slice
 * =========
 */
export const { actions, reducer } = createSlice({
  name: "chart",
  initialState: new BinaryChart(),
  reducers: {
    addWinRecord,
    addLoseRecord
  }
});

const store = configureStore({
  reducer
});

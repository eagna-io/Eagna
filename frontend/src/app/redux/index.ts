import { combineReducers, Store as ReduxStore } from "redux";

import * as prize from "./prize";

export type RootState = { prize: prize.State };

export type RootAction = prize.Action;

export const rootReducer = combineReducers<RootState, RootAction>({
  prize: prize.reducer
});

export type Store = ReduxStore<RootState, RootAction>;

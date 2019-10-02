import { combineReducers, Store as ReduxStore } from "redux";

import * as prize from "./prize";
import * as user from "./user";

export type RootState = { prize: prize.State; user: user.State };

export type RootAction = prize.Action | user.Action;

export const rootReducer = combineReducers<RootState, RootAction>({
  prize: prize.reducer,
  user: user.reducer
});

export type Store = ReduxStore<RootState, RootAction>;

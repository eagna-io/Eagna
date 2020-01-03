import { combineReducers, Store as ReduxStore } from "redux";

import * as user from "./user";

export type RootState = { user: user.State };

export type RootAction = user.Action;

export const rootReducer = combineReducers<RootState, RootAction>({
  user: user.reducer
});

export type Store = ReduxStore<RootState, RootAction>;

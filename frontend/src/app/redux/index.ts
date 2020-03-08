import { configureStore, combineReducers } from "@reduxjs/toolkit";

import { reducer as chartReducer } from "./chart";

const reducer = combineReducers({
  chart: chartReducer
});

export type RootState = ReturnType<typeof reducer>;

export const store = configureStore({
  reducer
});

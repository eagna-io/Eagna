import { Action as ReduxAction } from "redux";

export type Action<T extends string, Ext extends {} = {}> = ReduxAction<T> &
  { [K in keyof Ext]: Ext[K] };

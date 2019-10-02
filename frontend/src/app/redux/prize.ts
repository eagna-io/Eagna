import { ThunkAction } from "redux-thunk";

import { Action as AppAction } from "./commons";
import { Prize, PrizeRepository } from "models/prize";

export interface State {
  // まだ初回の取得を行なっていないとき、undefined をとる
  list: Prize[] | undefined;
}

export const INITIAL_STATE = {
  list: undefined
};

export enum ActionType {
  SetList = "SET_LIST"
}

export type Action = AppAction<ActionType.SetList, { list: Prize[] }>;

// Stateを直接更新する
export const setList = (list: Prize[]): Action => ({
  type: ActionType.SetList,
  list
});

// APIサーバーからPrizeのリストを取得し、Stateを更新する
export const queryPrizeList = (): ThunkAction<
  Promise<void>,
  State,
  null,
  Action
> => {
  return async dispatch => {
    const prizes = await PrizeRepository.queryAll();
    dispatch(setList(prizes));
  };
};

export const reducer = (
  state: State = INITIAL_STATE,
  action: Action
): State => {
  switch (action.type) {
    case ActionType.SetList:
      return {
        list: action.list
      };
    default:
      return state;
  }
};

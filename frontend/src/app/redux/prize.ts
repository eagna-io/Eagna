import { ThunkAction } from "redux-thunk";
import { Prize, PrizeRepository } from "models/prize";
import { Action as AppAction } from "./commons";
import { RootAction } from "./index";

export interface State {
  // まだ初回の取得を行なっていないとき、undefined をとる
  list: Prize[] | undefined;
}

export const INITIAL_STATE = {
  list: undefined
};

enum ActionType {
  SetList = "prize/SetList"
}

export type Action = AppAction<ActionType.SetList, { list: Prize[] }>;

function setList(list: Prize[]): Action {
  return {
    type: ActionType.SetList,
    list
  };
}

// APIサーバーからPrizeのリストを取得し、Stateを更新するAction
export function queryPrizeList(): ThunkAction<
  Promise<void>,
  State,
  null,
  Action
> {
  return async dispatch => {
    const prizes = await PrizeRepository.queryAll();
    dispatch(setList(prizes));
  };
}

export function reducer(
  state: State = INITIAL_STATE,
  action: RootAction
): State {
  switch (action.type) {
    case ActionType.SetList:
      return {
        list: action.list
      };
    default:
      return state;
  }
}

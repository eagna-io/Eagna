import { ThunkAction } from "redux-thunk";
import { Action as ReduxAction } from "redux";
import { Prize, PrizeRepository } from "models/prize";

export interface State {
  // まだ初回の取得を行なっていないとき、undefined をとる
  list: Prize[] | undefined;
}

export const INITIAL_STATE = {
  list: undefined
};

export type Action = SetListAction;

class SetListAction {
  type = "PRIZE_SET_LIST";
  constructor(readonly list: Prize[]) {}
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
    dispatch(new SetListAction(prizes));
  };
}

export function reducer<A extends ReduxAction>(
  state: State = INITIAL_STATE,
  action: A
): State {
  if (action instanceof SetListAction) {
    return {
      list: action.list
    };
  } else {
    return state;
  }
}

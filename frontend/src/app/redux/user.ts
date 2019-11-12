import { ThunkAction } from "redux-thunk";
import { User, UserRepository } from "models/user";
import { Action as AppAction } from "./commons";
import { RootAction } from "./index";

export interface State {
  // まだログイン状態かどうかわからないとき、undefined をとる
  user: User | null | undefined;
}

export const INITIAL_STATE = {
  user: undefined
};

enum ActionType {
  SetUser = "USER_SET_USER",
  ClearUser = "USER_CLEAR_USER"
}

export type Action =
  | AppAction<ActionType.SetUser, { user: User }>
  | AppAction<ActionType.ClearUser>;

function setUser(user: User): Action {
  return {
    type: ActionType.SetUser,
    user
  };
}

function clearUser(): Action {
  return {
    type: ActionType.ClearUser
  };
}

const LOGIN_CHECK_TIMEOUT_SEC = 10;

export function checkLogin(): ThunkAction<Promise<void>, State, null, Action> {
  return async dispatch => {
    const user = await UserRepository.queryMe();
    if (user) {
      dispatch(setUser(user));
    } else {
      dispatch(clearUser());
    }
  };
}

export function reducer(
  state: State = INITIAL_STATE,
  action: RootAction
): State {
  switch (action.type) {
    case ActionType.SetUser:
      return {
        user: action.user
      };
    case ActionType.ClearUser:
      return {
        user: null
      };
    default:
      return state;
  }
}

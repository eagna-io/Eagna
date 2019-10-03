import { ThunkAction } from "redux-thunk";
import * as firebase from "firebase/app";
import "firebase/auth";
import { User, UserRepository } from "models/user";
import { Action as AppAction } from "./commons";
import { RootAction } from './index';

export interface State {
  // まだログイン状態かどうかわからないとき、undefined をとる
  user: User | null | undefined;
}

export const INITIAL_STATE = {
  user: undefined
};

enum ActionType {
  SetUser = "USER_SET_USER",
  ClearUser = "USER_CLEAR_USER",
  ClearUserIfUndefined = "USER_CLEAR_USER_IF_UNDEFINED"
}

export type Action =
  | AppAction<ActionType.SetUser, { user: User }>
  | AppAction<ActionType.ClearUser>
  | AppAction<ActionType.ClearUserIfUndefined>;

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

function clearUserIfUndefined(): Action {
  return {
    type: ActionType.ClearUserIfUndefined
  };
}

const LOGIN_CHECK_TIMEOUT_MILLIS = 10 * 1000;

export function startObservingUserLogin(): ThunkAction<
  Promise<void>,
  State,
  null,
  Action
> {
  return async dispatch => {
    setTimeout(() => {
      dispatch(clearUserIfUndefined());
    }, LOGIN_CHECK_TIMEOUT_MILLIS);

    firebase.auth().onAuthStateChanged(async fbuser => {
      if (!fbuser) {
        dispatch(clearUser());
      }

      try {
        const user = await UserRepository.queryMe();
        if (user) {
          // ログイン済み
          dispatch(setUser(user));
        } else {
          // ここまでの処理の間に fbuser が null になった場合
          dispatch(clearUser());
        }
      } catch (e) {
        // 新規登録
        const user = await UserRepository.create();
        dispatch(setUser(user));
      }
    });
  };
}

export function reducer(state: State = INITIAL_STATE, action: RootAction): State {
  switch (action.type) {
    case ActionType.SetUser:
      return {
        user: action.user
      };
    case ActionType.ClearUser:
      return {
        user: null
      };
    case ActionType.ClearUserIfUndefined:
      if (state.user === undefined) {
        return {
          user: null
        };
      } else {
        return {
          ...state
        };
      }
    default:
      return state;
  }
}

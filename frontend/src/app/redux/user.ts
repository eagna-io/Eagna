import { ThunkAction } from "redux-thunk";
import { Action as ReduxAction } from "redux";
import * as firebase from "firebase/app";
import "firebase/auth";
import { User, UserRepository } from "models/user";

export interface State {
  // まだログイン状態かどうかわからないとき、undefined をとる
  user: User | null | undefined;
}

export const INITIAL_STATE = {
  user: undefined
};

export type Action =
  | SetUserAction
  | ClearUserAction
  | ClearUserIfUndefinedAction;

class SetUserAction {
  type = "USER_SET_USER";
  constructor(readonly user: User) {}
}

class ClearUserAction {
  type = "USER_CLEAR_USER";
}

class ClearUserIfUndefinedAction {
  type = "USER_CLEAR_USER_IF_UNDEFINED";
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
      dispatch(new ClearUserIfUndefinedAction());
    }, LOGIN_CHECK_TIMEOUT_MILLIS);

    firebase.auth().onAuthStateChanged(async fbuser => {
      if (!fbuser) {
        dispatch(new ClearUserAction());
      }

      try {
        const user = await UserRepository.queryMe();
        if (user) {
          // ログイン済み
          dispatch(new SetUserAction(user));
        } else {
          // ここまでの処理の間に fbuser が null になった場合
          dispatch(new ClearUserAction());
        }
      } catch (e) {
        // 新規登録
        const user = await UserRepository.create();
        dispatch(new SetUserAction(user));
      }
    });
  };
}

export function reducer<A extends ReduxAction>(
  state: State = INITIAL_STATE,
  action: A
): State {
  if (action instanceof SetUserAction) {
    return {
      user: action.user
    };
  } else if (action instanceof ClearUserAction) {
    return {
      user: null
    };
  } else if (action instanceof ClearUserIfUndefinedAction) {
    if (state.user === undefined) {
      return {
        user: null
      };
    } else {
      return {
        ...state
      };
    }
  } else {
    return state;
  }
}

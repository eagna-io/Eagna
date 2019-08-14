export const REQUEST_GET_ME = 'eagna/user/REQUEST_GET_ME';
export const SUCCESS_GET_ME = 'eagna/user/SUCCESS_GET_ME';
export const FAILURE_GET_ME = 'eagna/user/FAILURE_GET_ME';

export interface RequestGetMeAction {
  type: typeof REQUEST_GET;
  payload: {
    user_id: string;
  };
}

export interface SuccessGetMeAction {
  type: typeof SUCCESS_GET;
  payload: {
    user: User;
  };
}

export interface FailureGetMeAction {
  type: typeof FAILURE_GET;
  payload: Failure;
}

export type GetMeActions =
  | RequestGetMeAction
  | SuccessGetMeAction
  | FailureGetMeAction;

export function getMe(
  accessToken: string,
): ThunkAction<Promise<void>, UserState, {}, GetMeActions> {
  return dispatch => {
    dispatch(requestGet());
    return request({
      method: Method.GET,
      path: '/users',
      accessToken: accessToken,
      decoder: UserDecoder,
    }).then(res => {
      if (res instanceof Failure) {
        return failureGetMeAction(res.message);
      }
    });
  };
}

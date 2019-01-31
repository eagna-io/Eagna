import sha256 from 'js-sha256';

export const REQUEST_LOGIN = "RequestLogin";
export const RECEIVE_LOGIN_FAILED = "ReceiveLoginFailed";
export const RECEIVE_LOGIN_SUCCESS = "ReceiveLoginSuccess";
export const REQUEST_ME = "RequestMe";
export const RECEIVE_ME_FAILED = "ReceiveMeFailed";
export const RECEIVE_ME_SUCCESS = "ReceiveMeSuccess";

const apiBase = process.env.API_BASE;

export function requestLogin(name, rawPass) {
  return function(dispatch) {
    dispatch({
      type: REQUEST_LOGIN
    });
    const hashedPass = sha256(rawPass);
    return fetch(`${apiBase}/login?user=${name}&pass=${hashedPass}`)
      .then(res => res.json())
      .then(json => dispatch(receiveLogin(json)))
      .catch(err => dispatch({
          type: RECEIVE_LOGIN_FAILED,
          payload: "Connection is refused",
        })
      )
  }
}

export function receiveLogin(json) {
  if (json.success == false) {
    return {
      type: RECEIVE_LOGIN_FAILED,
      payload: json.result,
    };
  } else {
    return {
      type: RECEIVE_LOGIN_SUCCESS,
      payload: {
        accessToken: json.result,
      }
    };
  }
}

export function requestMe(token) {
  return function(dispatch) {
    dispatch({
      type: REQUEST_ME
    });
    return fetch(`${apiBase}/me?access_token=${token}`)
      .then(res => res.json())
      .then(json => dispatch(receiveMe(json)))
      .catch(err => dispatch({
          type: RECEIVE_ME_FAILED,
          payload: "Connection is refused",
        })
      )
  }
}

export function receiveMe(json) {
  if (json.success == false) {
    return {
      type: RECEIVE_ME_FAILED,
      payload: json.result,
    };
  } else {
    return {
      type: RECEIVE_ME_SUCCESS,
      payload: json.result,
    };
  }
}

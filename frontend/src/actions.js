import sha256 from 'js-sha256';

export const REQUEST_LOGIN = "RequestLogin";
export const RECEIVE_LOGIN_FAILED = "ReceiveLoginFailed";
export const RECEIVE_LOGIN_SUCCESS = "ReceiveLoginSuccess";
export const REQUEST_ME = "RequestMe";
export const RECEIVE_ME_FAILED = "ReceiveMe";
export const RECEIVE_ME_SUCCESS = "ReceiveMeSuccess";

const apiBase = process.env.API_BASE;

export function requestLogin(name, rawPass) {
  const action = {
    type: REQUEST_LOGIN
  };
  return function(dispatch) {
    dispatch(action);
    const hashedPass = sha256(rawPass);
    return fetch(`${apiBase}/login?user=${name}&pass=${hashedPass}`)
      .then(
        res => res.json(),
        err => console.log('Error while request login', err)
      )
      .then(json => dispatch(receiveLogin(json)))
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
  const action = {
    type: REQUEST_ME
  };
  return function(dispatch) {
    dispatch(action);
    return fetch(`${apiBase}/me?access_token=${token}`)
      .then(
        res => res.json(),
        err => console.log('Error while request me.', err)
      )
      .then(json => dispatch(receiveMe(json)))
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

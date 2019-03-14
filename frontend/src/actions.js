import sha256 from 'js-sha256';

export const REQUEST_LOGIN = "RequestLogin";
export const RECEIVE_LOGIN_FAILED = "ReceiveLoginFailed";
export const RECEIVE_LOGIN_SUCCESS = "ReceiveLoginSuccess";
export const REQUEST_ME = "RequestMe";
export const RECEIVE_ME_FAILED = "ReceiveMeFailed";
export const RECEIVE_ME_SUCCESS = "ReceiveMeSuccess";
export const REQUEST_MARKET = "RequestMarket";
export const RECEIVE_MARKET_FAILED = "ReceiveMarketFailed";
export const RECEIVE_MARKET_SUCCESS = "ReceiveMarketSuccess";
export const REQUEST_ORDER = "RequestOrder";
export const RECEIVE_ORDER_FAILED = "ReceiveOrderFailed";
export const RECEIVE_ORDER_SUCCESS = "ReceiveOrderSuccess";


const apiBase = process.env.API_BASE;


export function requestLogin(email, rawPass) {
  return function(dispatch) {
    dispatch({
      type: REQUEST_LOGIN
    });
    const hashedPass = sha256(rawPass);
    return fetch(`${apiBase}/login?email=${email}&pass=${hashedPass}`)
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

export function requestMarket(market_id, token) {
  return function(dispatch) {
    dispatch({
      type: REQUEST_MARKET
    });
    let url = `${apiBase}/markets/${market_id}`
    if (token != null) {
      url = url + `?access_token=${token}`
    }
    return fetch(url)
      .then(res => res.json())
      .then(json => dispatch(receiveMarket(json)))
      .catch(err => dispatch({
        type: RECEIVE_MARKET_FAILED,
        payload: "Connection is refused"
      }))
  }
}

export function receiveMarket(json) {
  if (json.success == false) {
    return {
      type: RECEIVE_MARKET_FAILED,
      payload: json.result,
    };
  } else {
    return {
      type: RECEIVE_MARKET_SUCCESS,
      payload: json.result,
    };
  }
}

export function requestOrder(tokenId, amountToken, amountCoin, marketId, accessToken) {
  return function(dispatch) {
    dispatch({
      type: REQUEST_ORDER
    });
    const url = `${apiBase}/order`;
    const method = "POST";
    const mode = "cors";
    const params = {
      "access_token": accessToken,
      "token_id": tokenId,
      "amount_token": amountToken,
      "amount_coin": amountCoin,
    };
    const body = JSON.stringify(params);
    const headers = {
      'Content-Type': 'application/json'
    };
    return fetch(url, {method, mode, headers, body})
      .then(res => res.json())
      .then(json => {
        dispatch(receiveOrder(json))
        dispatch(requestMarket(marketId, accessToken))
      })
      .catch(err => dispatch({
        type: RECEIVE_ORDER_FAILED,
        payload: "Connection is refused"
      }))
  }
}

export function receiveOrder(json) {
  if (json.success == false) {
    return {
      type: RECEIVE_ORDER_FAILED,
      payload: json.result,
    };
  } else {
    return {
      type: RECEIVE_ORDER_SUCCESS,
      payload: json.result,
    };
  }
}

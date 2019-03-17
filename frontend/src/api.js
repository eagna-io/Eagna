import sha256 from 'js-sha256';

export const ApiError = "ApiError";
export const NetworkError = "NetworkError";
export const LoginFailedError = "LoginFailedError";
export const InvalidAccessTokenError = "InvalidAccessTokenError";
export const MarketNotFoundError = "MarketNotFoundError";
export const TokenPriceIsMovedError = "TokenPriceIsMovedError";

const base = process.env.API_BASE;


export function getAccessToken(email, rawPass) {
  const hashedPass = sha256(rawPass);
  const url = `${base}/login?email=${email}&pass=${hashedPass}`;
  return get(url)
    .then(json => {
      if (!json.success) {
        if (json.result === "invalid email or password") {
          throw LoginFailedError;
        } else {
          getUnexpectedError(json.result);
        }
      }
      return json.result;
    })
}



export function getMe(token) {
  const url = `${base}/me?access_token=${token}`;
  return get(url)
    .then(json => {
      if (!json.success) {
        if (json.result === "invalid access token") {
          throw InvalidAccessTokenError;
        } else {
          getUnexpectedError(json.result);
        }
      }
      return json.result;
    });
}



export function getMarket(marketId, token = null) {
  let url = `${base}/markets/${marketId}`;
  if (token) {
    url = url + `?access_token=${token}`;
  }
  return get(url)
    .then(json => {
      if (!json.success) {
        if (json.result === "invalid access token") {
          throw InvalidAccessTokenError;
        } else if (json.result === "market not found"){
          throw MarketNotFoundError;
        } else {
          getUnexpectedError(json.result);
        }
      }
      return json.result;
    });
}



export function postOrder(tokenId, amountToken, amountCoin, accessToken) {
  const url = `${base}/order`;
  const params = {accessToken, tokenId, amountToken, amountCoin};
  return post(url, params)
    .then(json => {
      if (!json.success) {
        if (json.result === "access token is invalid") {
          throw InvalidAccessTokenError;
        } else if (json.result === "amount coin is invalid") {
          throw TokenPriceIsMovedError;
        } else {
          getUnexpectedError(json.result);
        }
      }
      return json.result;
    });
}



function get(url) {
  return fetch(url)
    .then(res => res.json())
    .catch(err => {
      throw NetworkError;
    })
}


function post(url, params) {
  const method = "POST";
  const mode = "cors";
  const body = JSON.stringify(params);
  const headers = {
    "Content-Type": "application/json"
  };
  return fetch(url, {method, mode, headers, body})
    .then(res => res.json())
    .catch(err => {
      throw NetworkError;
    });
}

function getUnexpectedError(err) {
  console.error("Get unexpected api error : %s", err);
  throw ApiError;
}

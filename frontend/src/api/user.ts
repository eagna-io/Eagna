import * as D from '@mojotech/json-type-validation';

import {request, Method, FailureCode, isFailure} from 'api/core';
import User from 'models/user';
import {Market} from 'models/market';
import {marketDecoder} from 'api/market';

export function getMe(accessToken: string): Promise<User | null> {
  return request({
    method: Method.GET,
    path: '/me/',
    accessToken: accessToken,
    decoder: userRespDecoder,
  }).then(res => {
    if (isFailure(res)) {
      if (res.error.code === FailureCode.Unauthorized) {
        return null;
      } else {
        throw `Unexpected failure : ${res.error.message}`;
      }
    } else {
      return {
        uid: res.id,
        name: res.name,
        email: res.email,
        isAdmin: res.isAdmin,
        accessToken: accessToken,
      };
    }
  });
}

export function getMyMarkets(accessToken: string): Promise<Market[]> {
  return request({
    method: Method.GET,
    path: '/me/markets/',
    accessToken: accessToken,
    decoder: D.array(marketDecoder),
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected failure : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

interface CreateUserArgs {
  accessToken: string;
  name: string;
  email: string;
}

export function createUser({
  accessToken,
  name,
  email,
}: CreateUserArgs): Promise<User> {
  return request({
    method: Method.POST,
    path: '/users/',
    accessToken: accessToken,
    decoder: userRespDecoder,
    body: {
      name: name,
      email: email,
    },
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected failure : ${res.error.message}`;
    } else {
      return {
        uid: res.id,
        name: res.name,
        email: res.email,
        isAdmin: res.isAdmin,
        accessToken: accessToken,
      };
    }
  });
}

interface UserResp {
  id: string;
  name: string;
  email: string;
  isAdmin: boolean;
}

const userRespDecoder: D.Decoder<UserResp> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
  isAdmin: D.boolean(),
});

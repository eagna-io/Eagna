import * as D from '@mojotech/json-type-validation';

import {request, Method, FailureCode, Failure} from 'api/core';
import {User} from 'models/user';

export function getMe(accessToken: string): Promise<User | 'Unauthorized'> {
  return request({
    method: Method.GET,
    path: '/users/me/',
    accessToken: accessToken,
    decoder: userDecoder,
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
    } else {
      return res;
    }
  });
}

export function createUser(
  accessToken: string,
  name: string,
  email: string,
): Promise<User> {
  return request({
    method: Method.POST,
    path: '/users/',
    accessToken: accessToken,
    decoder: userDecoder,
    body: {
      name: name,
      email: email,
    },
  }).then(res => {
    if (res instanceof Failure) {
      throw `Unexpected failure : ${res.message}`;
    } else {
      return res;
    }
  });
}

const userDecoder: D.Decoder<User> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
  isAdmin: D.boolean(),
}).map(obj => new User(id, name, email, isAdmin));

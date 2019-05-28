import * as D from '@mojotech/json-type-validation';

import {request, Method, FailureCode, isFailure} from 'api/core';
import User from 'models/user';

export function getMe(accessToken: string): Promise<User | null> {
  return request({
    method: Method.GET,
    path: '/me',
    accessToken: accessToken,
    decoder: getMeDecoder,
  }).then(res => {
    if (isFailure(res)) {
      if (res.error.code === FailureCode.Unauthorized) {
        return null;
      } else {
        throw {
          msg: `Unexpected failure : ${res.error.message}`,
        };
      }
    } else {
      return {
        uid: res.id,
        name: res.name,
        email: res.email,
        accessToken: accessToken,
      };
    }
  });
}

interface GetMe {
  id: string;
  name: string;
  email: string;
}

const getMeDecoder: D.Decoder<GetMe> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
});

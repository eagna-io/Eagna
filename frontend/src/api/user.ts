import * as D from '@mojotech/json-type-validation';

import {request, Method, FailureCode, isFailure} from 'api/core';
import User from 'models/user';
import Market from 'models/market';

export function getMe(accessToken: string): Promise<User | null> {
  return request({
    method: Method.GET,
    path: '/me',
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
        accessToken: accessToken,
      };
    }
  });
}

export function getMeMarkets(accessToken: string): Promise<Market[]> {
  return request({
    method: Method.GET,
    path: '/me/markets',
    accessToken: accessToken,
    decoder: marketsDecoder,
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected failure : ${res.error.message}`;
    } else {
      return res.map(m => ({
        id: m.id,
        title: m.title,
        organizer: m.organizer,
        short_desc: m.short_desc,
        description: m.description,
        open_time: new Date(m.open_time),
        close_time: new Date(m.close_time),
        tokens: m.tokens.map(t => ({
          id: t.id,
          name: t.name,
          description: t.description,
        })),
        status: m.status,
        settle_token_id: m.settle_token_id,
      }));
    }
  });
}

interface MarketResp {
  id: number;
  title: string;
  organizer: string;
  short_desc: string;
  description: string;
  open_time: string;
  close_time: string;
  tokens: {
    id: number;
    name: string;
    description: string;
  }[];
  status: string;
  settle_token_id?: number;
}

const marketsDecoder: D.Decoder<MarketResp[]> = D.array(
  D.object({
    id: D.number(),
    title: D.string(),
    organizer: D.string(),
    short_desc: D.string(),
    description: D.string(),
    open_time: D.string(),
    close_time: D.string(),
    tokens: D.array(
      D.object({
        id: D.number(),
        name: D.string(),
        description: D.string(),
      }),
    ),
    status: D.string(),
    settle_token_id: D.optional(D.number()),
  }),
);

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
    path: '/users',
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
        accessToken: accessToken,
      };
    }
  });
}

interface UserResp {
  id: string;
  name: string;
  email: string;
}

const userRespDecoder: D.Decoder<UserResp> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
});

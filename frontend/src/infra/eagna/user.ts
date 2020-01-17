import * as D from "@mojotech/json-type-validation";
import { EagnaBackendApi } from "infra/eagna";

export class EagnaUserApi {
  static queryMe(accessToken: string): Promise<User | null> {
    return EagnaBackendApi.get({
      path: "/users/me/",
      accessToken: accessToken,
      decoder: userDecoder
    }).catch(e => null);
  }

  static createAccessToken(args: {
    email: string;
    password: string;
  }): Promise<string | null> {
    return EagnaBackendApi.post({
      path: "/users/me/access_token/",
      decoder: D.object({ token: D.string() }),
      body: args
    })
      .then(({ token }) => token)
      .catch(e => null);
  }

  // 新規ユーザー登録を行う
  // 既に登録済みの場合はnull
  static create(user: {
    name: string;
    password: string;
    invitationToken: string;
  }): Promise<string | null> {
    return EagnaBackendApi.post({
      path: "/users/me/",
      decoder: D.object({ token: D.string() }),
      body: user
    })
      .then(({ token }) => token)
      .catch(e => null);
  }
}

export interface User {
  id: string;
  name: string;
  email: string;
  isAdmin: boolean;
  coin: number;
  point: number;
}

const userDecoder: D.Decoder<User> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
  isAdmin: D.boolean(),
  coin: D.number(),
  point: D.number(),
});

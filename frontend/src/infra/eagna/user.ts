import * as D from "@mojotech/json-type-validation";

import { EagnaBackendApi } from "infra/eagna";

export class EagnaUserApi {
  static queryMe(accessToken: string): Promise<User> {
    return EagnaBackendApi.get({
      path: "/users/me/",
      accessToken: accessToken,
      decoder: userDecoder
    });
  }

  static create(
    user: {
      name: string;
      email: string;
    },
    accessToken: string
  ): Promise<User> {
    return EagnaBackendApi.post({
      path: "/users/",
      accessToken: accessToken,
      decoder: userDecoder,
      body: user
    });
  }
}

export interface User {
  id: string;
  name: string;
  email: string;
  isAdmin: boolean;
}

const userDecoder: D.Decoder<User> = D.object({
  id: D.string(),
  name: D.string(),
  email: D.string(),
  isAdmin: D.boolean()
});

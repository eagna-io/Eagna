import * as D from "@mojotech/json-type-validation";

import * as http from "./";

export const post = async (name: string): Promise<PostRes> =>
  http.post({ path: "/accounts", body: { name }, decoder: PostResDecoder });

interface PostRes {
  access_token: string;
}

const PostResDecoder: D.Decoder<PostRes> = D.object({
  access_token: D.string()
});

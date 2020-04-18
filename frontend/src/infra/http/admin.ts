import * as D from "@mojotech/json-type-validation";

import * as http from "./";

export const post = async (email: string, pass: string): Promise<PostRes> =>
  http.post({
    path: "/admins/me/access_tokens",
    body: { email, pass },
    decoder: PostResDecoder
  });

interface PostRes {
  access_token: string;
}

const PostResDecoder: D.Decoder<PostRes> = D.object({
  access_token: D.string()
});

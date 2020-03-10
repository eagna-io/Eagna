import { RPC_URL } from "../rpc";

export interface Params {
  marketId: string,
  outcomeId: string,
  accountName: string,
}

export const vote = (params: Params): Promise<void> => {
  console.log("vote rpc");
  return fetch(RPC_URL, {
    method: "POST",
    body: JSON.stringify(params),
    headers: {
      "Content-Type": "application/json"
    }
  })
    .then(res => res.json())
    .then(json => console.log(json))
};

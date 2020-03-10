import * as D from "@mojotech/json-type-validation";

export const RPC_URL = process.env.REACT_APP_API_BASE + "/rpc";

export const call = <T, S>(
  method: string,
  params: T,
  decoder: D.Decoder<S>
): Promise<S> => {
  const body: Request<T> = {
    jsonrpc: "2.0",
    id: Math.floor(Math.random() * 1000000),
    method,
    params
  };

  return fetch(RPC_URL, {
    method: "POST",
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json"
    }
  })
    .then(res => res.json())
    .then(json => ResponseDecoder(decoder).runPromise(json))
    .then(res => {
      if (isSuccess(res)) {
        return res.result;
      } else {
        throw new Error(`RPC Failure : ${res.error.message}`);
      }
    });
};

export const isSuccess = <S>(res: Response<S>): res is Success<S> => {
  return "error" in res;
};

export interface Request<T> {
  jsonrpc: "2.0";
  id: number;
  method: string;
  params: T;
}

export type Response<S> = Success<S> | Failure;

export interface Success<S> {
  jsonrpc: "2.0";
  id: number;
  result: S;
}

export interface Failure {
  jsonrpc: "2.0";
  id: number;
  error: {
    code: number;
    message: string;
  };
}

const ResponseDecoder = <S>(
  resultDecoder: D.Decoder<S>
): D.Decoder<Response<S>> =>
  D.union(SuccessDecoder(resultDecoder), FailureDecoder());

const SuccessDecoder = <S>(
  resultDecoder: D.Decoder<S>
): D.Decoder<Success<S>> =>
  D.object({
    jsonrpc: D.constant<"2.0">("2.0"),
    id: D.number(),
    result: resultDecoder
  });

const FailureDecoder = (): D.Decoder<Failure> =>
  D.object({
    jsonrpc: D.constant<"2.0">("2.0"),
    id: D.number(),
    error: D.object({
      code: D.number(),
      message: D.string()
    })
  });

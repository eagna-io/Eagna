import * as D from '@mojotech/json-type-validation';

const base = process.env.REACT_APP_API_BASE;

export enum Method {
  GET = 'GET',
  POST = 'POST',
}

export interface RequestParams<T> {
  method: Method;
  path: string;
  accessToken?: string;
  params?: object;
  body?: object;
  decoder: D.Decoder<T>;
}

export interface Failure {
  error: {
    code: number;
    message: string;
  };
}

export enum FailureCode {
  ResourceNotFound = 0,
  InvalidPayload = 1,
  Unauthorized = 2,
  ServerError = 100,
}

export function isFailure<T>(v: T | Failure): v is Failure {
  return (v as Failure).error !== undefined;
}

export function request<T>(args: RequestParams<T>): Promise<T | Failure> {
  const url = constructUrl(args.path, args.params);
  let option = {
    method: args.method,
    headers: new Headers({
      Accept: 'application/json',
    }),
  };
  if (args.accessToken != null) {
    option.headers.set('Authorization', `Bearer: ${args.accessToken}`);
  }
  if (args.body != null) {
    option = Object.assign({body: JSON.stringify(args.body)}, option);
    option.headers.set('Content-Type', 'application/json');
  }
  return fetch(url, option).then(res => {
    if (res.ok) {
      return res.json().then(json => args.decoder.runWithException(json));
    } else {
      return res.json().then(json => failureDecoder.runWithException(json));
    }
  });
}

function constructUrl(path: string, params?: object): string {
  let url = `https://${base}${path}`;
  if (params == null) {
    return url;
  } else {
    url += '?';
    Object.entries(params).forEach((key, val) => {
      url += `${key}=${val}&`;
    });
    return url.slice(0, -1);
  }
}

const failureDecoder: D.Decoder<Failure> = D.object({
  error: D.object({
    code: D.number(),
    message: D.string(),
  }),
});

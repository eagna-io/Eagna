import * as D from '@mojotech/json-type-validation';

const base = process.env.REACT_APP_API_BASE;

export enum Method {
  GET = 'GET',
  POST = 'POST',
  PUT = 'PUT',
}

export interface RequestArgs<T> {
  method: Method;
  path: string;
  accessToken?: string;
  params?: RequestParams;
  body?: object;
  decoder: D.Decoder<T>;
}

export interface RequestParams {
  [key: string]: RequestParamVal;
}

type RequestParamVal = string | number | boolean | Array<string | number>;

export class Failure {
  constructor(readonly code: number, readonly message: string) {}
}

export enum FailureCode {
  ResourceNotFound = 0,
  InvalidPayload = 1,
  Unauthorized = 2,
  ServerError = 100,
}

export function request<T>(args: RequestArgs<T>): Promise<T | Failure> {
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

function constructUrl(path: string, params?: RequestParams): string {
  let url = `${base}${path}`;
  if (params == null) {
    return url;
  } else {
    url += '?';
    Object.entries(params).forEach(([key, val]) => {
      url += `${key}=${serializeParamVal(val)}&`;
    });
    return url.slice(0, -1);
  }
}

// 配列の場合は、","区切りで値を並べ、それをパーセントエンコードする。
// つまり、"%2C"で区切られる
function serializeParamVal(v: RequestParamVal): string {
  if (typeof v === 'string') {
    return v;
  } else if (typeof v === 'number') {
    return String(v);
  } else if (typeof v === 'boolean') {
    return String(v);
  } else {
    return v
      .map(toString)
      .reduce((acc, cur) => acc + cur + '%2C', '')
      .slice(0, -3);
  }
}

function toString(v: string | number): string {
  if (typeof v === 'string') {
    return v;
  } else {
    return String(v);
  }
}

const failureDecoder: D.Decoder<Failure> = D.object({
  error: D.object({
    code: D.number(),
    message: D.string(),
  }),
}).map(obj => new Failure(obj.error.code, obj.error.message));

import * as D from "@mojotech/json-type-validation";

const base = process.env.REACT_APP_API_BASE;

export enum Method {
  GET = "GET",
  POST = "POST",
  PUT = "PUT"
}

export interface RequestArgs<T> {
  path: string;
  accessToken?: string;
  params?: RequestParams;
  body?: object;
  decoder: D.Decoder<T>;
}

export class EagnaBackendApi {
  static get<T>(args: RequestArgs<T>): Promise<T> {
    return EagnaBackendApi.request(Method.GET, args);
  }

  static post<T>(args: RequestArgs<T>): Promise<T> {
    return EagnaBackendApi.request(Method.POST, args);
  }

  static put<T>(args: RequestArgs<T>): Promise<T> {
    return EagnaBackendApi.request(Method.PUT, args);
  }

  private static async request<T>(
    method: Method,
    args: RequestArgs<T>
  ): Promise<T> {
    const err = new Error();
    const url = constructUrl(args.path, args.params);
    let option = {
      method: method,
      headers: new Headers({
        Accept: "application/json"
      })
    };
    if (args.accessToken != null) {
      option.headers.set("Authorization", `Bearer: ${args.accessToken}`);
    }
    if (args.body != null) {
      option = Object.assign({ body: JSON.stringify(args.body) }, option);
      option.headers.set("Content-Type", "application/json");
    }
    const res = await fetch(url, option);
    const json = await res.json();
    if (res.ok) {
      return args.decoder.runWithException(json);
    } else {
      const failure = failureDecoder.runWithException(json);
      err.message = failure.message;
      throw err;
    }
  }
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
  ServerError = 100
}

function constructUrl(path: string, params?: RequestParams): string {
  let url = `${base}${path}`;
  if (params == null) {
    return url;
  } else {
    url += "?";
    Object.entries(params).forEach(([key, val]) => {
      url += `${key}=${serializeParamVal(val)}&`;
    });
    return url.slice(0, -1);
  }
}

// 配列の場合は、","区切りで値を並べ、それをパーセントエンコードする。
// つまり、"%2C"で区切られる
function serializeParamVal(v: RequestParamVal): string {
  if (typeof v === "string") {
    return v;
  } else if (typeof v === "number") {
    return String(v);
  } else if (typeof v === "boolean") {
    return String(v);
  } else {
    return v
      .map(toString)
      .reduce((acc, cur) => acc + cur + "%2C", "")
      .slice(0, -3);
  }
}

function toString(v: string | number): string {
  if (typeof v === "string") {
    return v;
  } else {
    return String(v);
  }
}

const failureDecoder: D.Decoder<Failure> = D.object({
  error: D.object({
    code: D.number(),
    message: D.string()
  })
}).map(obj => new Failure(obj.error.code, obj.error.message));


interface RequestParams {
  method: Method,
  path: string,
  accessToken?: string,
  params?: object,
  body?: object,
}

export function request<T>(args: RequestParams, decoder: Decoder<T>): Promise<T> {
}

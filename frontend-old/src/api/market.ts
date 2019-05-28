import {Method, request} from 'api/core';
import {Market} from 'models/market';

export function getMarket(
  marketId: number,
  accessToken?: string,
): Promise<Market> {
  return request({
    method: Method.Get,
    path: `/markets/${marketId}`,
    accessToken: accessToken,
  });
}

import {
  EagnaUserApi,
  MarketRewardRecord as InfraMarketRewardRecord,
  PrizeTradeRecord as InfraPrizeTradeRecord
} from "infra/eagna/user";
import { Storage } from "infra/storage";

// 現在ログインしているユーザー
export class User {
  constructor(
    readonly uid: string,
    readonly name: string,
    readonly email: string,
    readonly isAdmin: boolean,
    readonly point: number,
    readonly prizeTradeHistory: PrizeTradeRecord[],
    readonly marketRewardHistory: MarketRewardRecord[],
    readonly accessToken: string
  ) {}

  // 後々、accessTokenがexpireしていた場合に
  // refreshTokenを使用してaccessTokenを再取得する
  // 場合などが想定されるため、専用関数でラップしている
  async getAccessToken(): Promise<string> {
    return this.accessToken;
  }
}

export type PrizeTradeRecord = InfraPrizeTradeRecord;
export type MarketRewardRecord = InfraMarketRewardRecord;

export class UserRepository {
  static async queryMe(): Promise<User | null> {
    const token = Storage.getToken();
    if (!token) {
      return null;
    } else {
      try {
        const user = await EagnaUserApi.queryMe(token);
        return new User(
          user.id,
          user.name,
          user.email,
          user.isAdmin,
          user.point,
          user.prizeTradeHistory,
          user.marketRewardHistory,
          token
        );
      } catch (e) {
        // Tokenが期限切れの時とか
        console.log(e);
        return null;
      }
    }
  }
}

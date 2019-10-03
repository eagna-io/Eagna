import * as firebase from "firebase/app";
import "firebase/auth";
import {
  EagnaUserApi,
  MarketRewardItem as InfraMarketRewardItem,
  PrizeTradeItem as InfraPrizeTradeItem
} from "infra/eagna/user";

// 現在ログインしているユーザー
export class User {
  readonly point: number;

  constructor(
    readonly uid: string,
    readonly name: string,
    readonly email: string,
    readonly isAdmin: boolean,
    readonly pointHistory: (MarketRewardItem | PrizeTradeItem)[]
  ) {
    this.point = pointHistory.reduce((sum, item) => {
      if (item.type === "MarketReward") {
        return sum + item.point;
      } else {
        // item.type === PrizeTrade のとき
        return sum - item.point;
      }
    }, 0);
  }

  getAccessToken(): Promise<string> {
    const fbuser = firebase.auth().currentUser;
    if (!fbuser) {
      throw new Error("User does not logged in");
    } else if (fbuser.uid !== this.uid) {
      throw new Error(`User ${this.uid} is not logged in user`);
    } else {
      return fbuser.getIdToken();
    }
  }
}

export type MarketRewardItem = InfraMarketRewardItem;
export type PrizeTradeItem = InfraPrizeTradeItem;

export class UserRepository {
  static async queryMe(): Promise<User | null> {
    const fbuser = firebase.auth().currentUser;
    if (!fbuser) {
      return null;
    } else {
      const accessToken = await fbuser.getIdToken();
      const user = await EagnaUserApi.queryMe(accessToken);
      return new User(
        user.id,
        user.name,
        user.email,
        user.isAdmin,
        user.pointHistory
      );
    }
  }

  static async create(): Promise<User> {
    const fbuser = firebase.auth().currentUser;
    if (!fbuser || !fbuser.displayName || !fbuser.email) {
      console.log(fbuser);
      throw new Error(`invalid firebase user : ${fbuser}`);
    } else {
      const newUser = {
        name: fbuser.displayName,
        email: fbuser.email
      };
      const accessToken = await fbuser.getIdToken();
      const user = await EagnaUserApi.create(newUser, accessToken);
      return new User(
        user.id,
        user.name,
        user.email,
        user.isAdmin,
        user.pointHistory
      );
    }
  }
}

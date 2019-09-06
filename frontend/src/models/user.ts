import * as firebase from "firebase/app";
import "firebase/auth";

import { EagnaUserApi } from "infra/eagna/user";

// 現在ログインしているユーザー
export class User {
  constructor(
    readonly uid: string,
    readonly name: string,
    readonly email: string,
    readonly isAdmin: boolean
  ) {}

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

export class UserRepository {
  static async queryMe(): Promise<User | null> {
    const fbuser = firebase.auth().currentUser;
    if (!fbuser) {
      return null;
    } else {
      const accessToken = await fbuser.getIdToken();
      const user = await EagnaUserApi.queryMe(accessToken);
      return new User(user.id, user.name, user.email, user.isAdmin);
    }
  }

  static async create(): Promise<User | null> {
    const fbuser = firebase.auth().currentUser;
    if (!fbuser || !fbuser.displayName || !fbuser.email) {
      return null;
    } else {
      const newUser = {
        name: fbuser.displayName,
        email: fbuser.displayName
      };
      const accessToken = await fbuser.getIdToken();
      const user = await EagnaUserApi.create(newUser, accessToken);
      return new User(user.id, user.name, user.email, user.isAdmin);
    }
  }
}

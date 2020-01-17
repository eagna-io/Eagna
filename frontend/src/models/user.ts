import {
  EagnaUserApi,
  User as InfraUser,
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
    readonly accessToken: string
  ) {}

  // 後々、accessTokenがexpireしていた場合に
  // refreshTokenを使用してaccessTokenを再取得する
  // 場合などが想定されるため、専用関数でラップしている
  async getAccessToken(): Promise<string> {
    return this.accessToken;
  }

  static fromInfra(user: InfraUser, token: string): User {
    return new User(
      user.id,
      user.name,
      user.email,
      user.isAdmin,
      user.point,
      token
    );
  }
}

export class UserRepository {
  static async queryMe(): Promise<User | null> {
    const token = Storage.getToken();
    if (!token) {
      return null;
    } else {
      const user = await EagnaUserApi.queryMe(token);
      if (user === null) {
        return null;
      } else {
        return User.fromInfra(user, token);
      }
    }
  }

  static async signin(email: string, password: string): Promise<User | null> {
    const token = await EagnaUserApi.createAccessToken({
      email,
      password
    });
    if (!token) {
      // 認証失敗
      return null;
    }

    // Userをfetch
    const user = await EagnaUserApi.queryMe(token);
    if (!user) {
      // この場合、おそらくサーバーに何らかのバグがある
      console.error("Success to create access token but it seems to invalid");
      return null;
    }

    // Tokenを保存
    Storage.setToken(token);

    return User.fromInfra(user, token);
  }
}

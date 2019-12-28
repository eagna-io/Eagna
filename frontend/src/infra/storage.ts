// localStorage はドメインローカルだが、XSSに対する脆弱性は残る。
// TODO : localStorageを使わず、httpOnlyなCookieに
// トークンを保存する。

const TokenKey = "4YwUgrFlBNgIB4DnGprvPFgPWQoPGrSB";

export class Storage {
  static getToken(): string | null {
    return localStorage.getItem(TokenKey);
  }

  static setToken(token: string) {
    localStorage.setItem(TokenKey, token);
  }

  static removeToken() {
    localStorage.removeItem(TokenKey);
  }
}

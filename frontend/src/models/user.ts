import * as firebase from 'firebase/app';
import 'firebase/auth';

export class User {
  constructor(
    readonly uid: string,
    readonly name: string,
    readonly email: string,
    readonly isAdmin: boolean,
  ) {}

  getAccessToken(): Promise<string | null> {
    const fbUser = firebase.auth().currentUser;
    if (fbUser !== null && fbUser.uid === this.uid) {
      return fbUser.getIdToken();
    } else {
      return Promise.resolve(null);
    }
  }
}

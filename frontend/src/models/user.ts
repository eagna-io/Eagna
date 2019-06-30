import * as firebase from 'firebase/app';
import 'firebase/auth';

export interface User {
  uid: string;
  name: string;
  email: string;
  isAdmin: boolean;
}

export function getAccessToken(user: User): Promise<string | null> {
  const fbUser = firebase.auth().currentUser;
  if (fbUser === null) {
    return Promise.resolve(null);
  } else {
    return fbUser.getIdToken();
  }
}

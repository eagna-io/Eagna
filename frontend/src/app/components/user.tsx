import React, {FC, useState, useEffect, createContext, useContext} from 'react';
import * as firebase from 'firebase/app';
import 'firebase/auth';

import {User} from 'models/user';
import {getMe, createUser} from 'api/user';

const LOGIN_CHECK_SEC = 5;

export type LoginStatus = 'Checking' | User | null;

export const UserContext = createContext<LoginStatus>('Checking');

export const UserProvider: React.FC = ({children}) => {
  const [fbuser, setFbuser] = useState<firebase.User | null>(null);
  const [loginStatus, setLoginStatus] = useState<LoginStatus>('Checking');
  console.log(loginStatus);

  useEffect(() => {
    const unsubscribe = firebase.auth().onAuthStateChanged(fbuser => {
      setFbuser(fbuser);
    });
    return unsubscribe;
  }, []);

  useEffect(() => {
    if (loginStatus === 'Checking') {
      if (fbuser !== null) {
        // 'Checking' から User へ移行するパターン
        fbuser.getIdToken().then(token =>
          getMe(token).then(user => {
            if (user instanceof User) {
              // すでに登録済み。そのままログインする。
              setLoginStatus(user);
            } else {
              // サーバーにまだ登録されていない。まず登録する。
              if (fbuser.displayName === null || fbuser.email === null) {
                // TODO : 名前とメアドを取得するプロセスを追加する
                throw new Error('Cant get name or email from Firebase Auth');
              } else {
                createUser(token, fbuser.displayName, fbuser.email).then(user =>
                  setLoginStatus(user),
                );
              }
            }
          }),
        );
      } else {
        // 'Checking' から null へ移行するパターン
        const timer = setTimeout(() => {
          if (loginStatus === 'Checking') {
            setLoginStatus(null);
          }
        }, LOGIN_CHECK_SEC * 1000);

        return () => clearTimeout(timer);
      }
    } else if (loginStatus === null) {
      // null から直接 User へ移行するパターンはない
      // 'Checking' を経由する
      if (fbuser !== null) {
        setLoginStatus('Checking');
      }
    } else if (loginStatus instanceof User) {
      // User から 直接 'Checking' へ移行するパターンはない
      // null を経由する
      if (fbuser === null) {
        setLoginStatus(null);
      }
    }
  }, [fbuser, loginStatus]);

  return (
    <UserContext.Provider value={loginStatus}>{children}</UserContext.Provider>
  );
};

export interface UserHOCInjectProps {
  user: LoginStatus;
}

export function withUser<P extends {}>(
  WrappedComponent: React.FC<P & UserHOCInjectProps>,
): React.FC<P> {
  const UserHOC: FC<P> = props => {
    const loginStatus = useContext(UserContext);

    return <WrappedComponent user={loginStatus} {...props} />;
  };

  return UserHOC;
}

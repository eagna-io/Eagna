import React, {
  FC,
  useState,
  useEffect,
  createContext,
  useContext
} from "react";
import * as firebase from "firebase/app";
import "firebase/auth";

import { User, UserRepository } from "models/user";

const LOGIN_CHECK_SEC = 5;

export type LoginStatus = "Checking" | User | null;

export const UserContext = createContext<LoginStatus>("Checking");

export const UserProvider: React.FC = ({ children }) => {
  const [fbuser, setFbuser] = useState<firebase.User | null>(null);
  const [loginStatus, setLoginStatus] = useState<LoginStatus>("Checking");

  useEffect(() => {
    const unsubscribe = firebase.auth().onAuthStateChanged(fbuser => {
      console.log(fbuser);
      setFbuser(fbuser);
    });
    return unsubscribe;
  }, []);

  useEffect(() => {
    if (loginStatus === "Checking") {
      if (fbuser !== null) {
        (async () => {
          // 'Checking' から User へ移行するパターン
          try {
            const user = await UserRepository.queryMe();
            setLoginStatus(user);
          } catch (e) {
            const user = await UserRepository.create();
            setLoginStatus(user);
          }
        })();
      } else {
        // 'Checking' から null へ移行するパターン
        const timer = setTimeout(() => {
          if (loginStatus === "Checking") {
            setLoginStatus(null);
          }
        }, LOGIN_CHECK_SEC * 1000);

        return () => clearTimeout(timer);
      }
    } else if (loginStatus === null) {
      // null から直接 User へ移行するパターンはない
      // 'Checking' を経由する
      if (fbuser !== null) {
        setLoginStatus("Checking");
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

export interface UserProps {
  user: LoginStatus;
}

export function withUser<P extends UserProps>(
  WrappedComponent: React.FC<P>
): React.FC<Omit<P, keyof UserProps>> {
  const UserHOC: FC<Omit<P, keyof UserProps>> = props => {
    const loginStatus = useContext(UserContext);

    return <WrappedComponent user={loginStatus} {...(props as any)} />;
  };

  return UserHOC;
}

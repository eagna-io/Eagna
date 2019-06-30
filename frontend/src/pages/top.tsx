import React, {FC, useState, useEffect, useMemo} from 'react';
import * as firebase from 'firebase/app';
import 'firebase/auth';
import {History} from 'history';

import {Market, MarketStatus} from 'models/market';
import {User} from 'models/user';
import {getMarkets} from 'api/market';
import {getMe, createUser} from 'api/user';
import {Pc, Tablet, Mobile} from 'components/responsive';
import TopPagePc from './top/pc';
import TopPageMobile from './top/mobile';

interface TopPageProps {
  history: History;
  setUser: (user: User) => void;
}

const TopPage: FC<TopPageProps> = ({history, setUser}) => {
  const [featuredMarkets, setFeaturedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    getMarkets([MarketStatus.Upcoming, MarketStatus.Open]).then(res =>
      setFeaturedMarkets(res),
    );
  }, []);

  const authConfig = useMemo(
    () => ({
      callbacks: {
        signInSuccessWithAuthResult: (args: {user: firebase.User}) => {
          const fbUser = args.user;
          fbUser
            .getIdToken()
            .then(token =>
              getMe(token).then(maybeUser => {
                if (maybeUser !== null) {
                  return maybeUser;
                } else {
                  // Firebase認証は終わっているが、サーバーには登録されていない
                  if (fbUser.displayName === null || fbUser.email === null) {
                    // TODO
                    throw 'Cant get name or email from Firebase Auth';
                  } else {
                    return createUser({
                      accessToken: token,
                      name: fbUser.displayName,
                      email: fbUser.email,
                    });
                  }
                }
              }),
            )
            .then(user => {
              setUser(user);
              history.push('/me');
            });

          return false;
        },
      },
      signInSuccessUrl: '/me',
      signInOptions: [
        {
          provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
          scopes: ['https://www.googleapis.com/auth/userinfo.email'],
          customParameters: {
            prompt: 'select_account',
          },
        },
        {
          provider: firebase.auth.FacebookAuthProvider.PROVIDER_ID,
          scopes: ['email'],
        },
        {
          provider: firebase.auth.GithubAuthProvider.PROVIDER_ID,
          scopes: ['user:email'],
        },
        {
          provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
          requireDisplayName: true,
        },
      ],
    }),
    [history, setUser],
  );

  const mobile = (
    <TopPageMobile uiConfig={authConfig} featuredMarkets={featuredMarkets} />
  );

  const pc = (
    <TopPagePc uiConfig={authConfig} featuredMarkets={featuredMarkets} />
  );

  return (
    <>
      <Mobile>{mobile}</Mobile>
      <Tablet>{mobile}</Tablet>
      <Pc>{pc}</Pc>
    </>
  );
};

export default TopPage;

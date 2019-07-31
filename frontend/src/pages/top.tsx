import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import * as firebase from 'firebase/app';
import 'firebase/auth';
import StyledFirebaseAuth from 'react-firebaseui/StyledFirebaseAuth';
import {History} from 'history';

import {Market} from 'models/market';
import {User} from 'models/user';
import {getMarkets} from 'api/market';
import {getMe, createUser} from 'api/user';
import {pc} from 'components/responsive';

import Header from './top/components/header';
import FeaturedMarketComponent from './top/components/featured_market';
import Footer from './top/components/footer';

interface TopPageProps {
  history: History;
  setUser: (user: User) => void;
}

const TopPage: FC<TopPageProps> = ({history, setUser}) => {
  const [featuredMarkets, setFeaturedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    getMarkets(['Upcoming', 'Open']).then(res => setFeaturedMarkets(res));
  }, []);

  const authConfig = createAuthConfig(history, setUser);

  return (
    <>
      <Header />
      <MainSection>
        <MainSectionBgFilter>
          <MainMsg>未来は僕等の手の中</MainMsg>
          <SubMsg>今すぐ予測市場を体験しましょう</SubMsg>
          <StyledFirebaseAuth
            uiConfig={authConfig}
            firebaseAuth={firebase.auth()}
          />
          <AnnounceBetaRelease href="https://note.mu/rohan_market/n/n7f8a517c50f6">
            &beta; 版をリリースしました！
          </AnnounceBetaRelease>
        </MainSectionBgFilter>
      </MainSection>
      <FeaturedMarketsSection>
        <SectionTitle>注目のマーケット</SectionTitle>
        <FeaturedMarketList>
          {featuredMarkets.map(m => (
            <FeaturedMarketComponent key={m.id} market={m} />
          ))}
        </FeaturedMarketList>
      </FeaturedMarketsSection>
      <Footer />
    </>
  );
};

export default TopPage;

function createAuthConfig(
  history: History,
  setUser: (user: User) => void,
): object {
  return {
    callbacks: {
      signInSuccessWithAuthResult: (args: {user: firebase.User}) => {
        const fbUser = args.user;
        fbUser
          .getIdToken()
          .then(token =>
            getMe(token).then(maybeUser => {
              if (maybeUser instanceof User) {
                return maybeUser;
              } else {
                // Firebase認証は終わっているが、サーバーには登録されていない
                if (fbUser.displayName === null || fbUser.email === null) {
                  // TODO
                  throw new Error('Cant get name or email from Firebase Auth');
                } else {
                  return createUser(token, fbUser.displayName, fbUser.email);
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
  };
}

const MainSection = styled.div`
  height: 448px;
  background-image: url('/img/top/main-bg.jpg');
  background-position: center;
  background-size: cover;

  ${pc(`
    height: 787px;
  `)}
`;

const MainSectionBgFilter = styled.div`
  width: 100%;
  height: 100%;
  background-color: rgba(255, 255, 255, 0.7);
  padding-top: 80px;

  ${pc(`
    padding-top: 200px;
  `)}
`;

const MainMsg = styled.h2`
  height: 22px;
  width: 100%;
  font-size: 20px;
  font-weight: 400;
  line-height: 22px;
  text-align: center;
  margin: 0;
  padding: 0;

  ${pc(`
    height: 54px;
    font-size: 36px;
    line-height: 54px;
  `)}
`;

const SubMsg = styled.h3`
  height: 24px;
  width: 100%;
  top: 12px;
  font-size: 16px;
  font-weight: 300;
  text-align: center;
  line-height: 24px;
  margin: 0;
  margin-top: 13px;
  margin-bottom: 15px;
  padding: 0;

  ${pc(`
    height: 45px;
    font-size 30px;
    line-height: 45px;
  `)}
`;

const AnnounceBetaRelease = styled.a`
  display: block;
  width: 100%;
  margin-top: 30px;
  font-size: 15px;
  font-weight: 400;
  text-align: center;
  text-decoration: underline;

  ${pc(`
    width: 400px;
    margin: 0 auto;
    margin-top: 50px;
    font-size: 18px;
  `)}
`;

const FeaturedMarketsSection = styled.div`
  width: 100vw;
  padding-top: 31px;
  padding-bottom: 50px;

  ${pc(`
    padding-top: 64px;
    padding-bottom: 183px;
  `)}
`;

const FeaturedMarketList = styled.div`
  width: 100vw;
  margin: 0 auto;

  ${pc(`
    width: 980px;
    margin: 0 auto;
  `)}
`;

const SectionTitle = styled.h3`
  width: 100%;
  height: 30px;
  text-align: center;
  line-height: 30px;
  font-size: 20px;
  font-weight: 100;
  text-align: center;
  margin: 0;
  padding: 0;

  ${pc(`
    height: 54px;
    line-height: 54px;
    font-size: 36px;
  `)}
`;

import React, {FC, useState, useEffect} from 'react';
import firebase from 'firebase';

import {Market, MarketStatus} from 'models/market';
import {getMarkets} from 'api/market';
import {Pc, Tablet, Mobile} from 'components/responsive';
import TopPagePc from './top/pc';
import TopPageMobile from './top/mobile';

interface TopPageProps {}

const TopPage: FC<TopPageProps> = () => {
  const [featuredMarkets, setFeaturedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    getMarkets([MarketStatus.Upcoming, MarketStatus.Open]).then(res =>
      setFeaturedMarkets(res),
    );
  }, []);

  const mobile = (
    <TopPageMobile uiConfig={uiConfig} featuredMarkets={featuredMarkets} />
  );

  const pc = (
    <TopPagePc uiConfig={uiConfig} featuredMarkets={featuredMarkets} />
  );

  return (
    <>
      <Mobile>{mobile}</Mobile>
      <Tablet>{mobile}</Tablet>
      <Pc>{pc}</Pc>
    </>
  );
};

const uiConfig = {
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

export default TopPage;

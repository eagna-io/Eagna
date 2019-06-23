import React, {FC} from 'react';
import styled from 'styled-components';
import StyledFirebaseAuth from 'react-firebaseui/StyledFirebaseAuth';
import firebase from 'firebase';

import {Market} from 'models/market';
import * as Header from './components/header';
import * as FeaturedMarketComponent from './components/featured_market';
import * as Footer from './components/footer';

interface TopPageProps {
  uiConfig: any;
  featuredMarkets: Market[];
}

export const TopPageMobile: FC<TopPageProps> = ({
  uiConfig,
  featuredMarkets,
}) => {
  return (
    <>
      <Header.Mobile />
      <MainSection>
        <MainSectionBgFilter>
          <MainMsg>未来は僕等の手の中</MainMsg>
          <SubMsg>今すぐ予測市場を体験しましょう</SubMsg>
          <StyledFirebaseAuth
            uiConfig={uiConfig}
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
            <FeaturedMarketComponent.Mobile key={m.id} market={m} />
          ))}
        </FeaturedMarketList>
      </FeaturedMarketsSection>
      <Footer.Mobile />
    </>
  );
};

export default TopPageMobile;

const MainSection = styled.div`
  height: 448px;
  background-image: url('/img/top/main-bg.png');
  background-position: center;
  background-size: cover;
`;

const MainSectionBgFilter = styled.div`
  width: 100%;
  height: 100%;
  background-color: rgba(255, 255, 255, 0.7);
  padding-top: 80px;
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
`;

const AnnounceBetaRelease = styled.a`
  display: block;
  width: 100%;
  margin-top: 30px;
  font-size: 15px;
  font-weight: 400;
  text-align: center;
  text-decoration: underline;
`;

const FeaturedMarketsSection = styled.div`
  width: 100vw;
  padding-top: 31px;
  padding-bottom: 50px;
`;

const FeaturedMarketList = styled.div`
  width: 100vw;
  margin: 0 auto;
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
`;

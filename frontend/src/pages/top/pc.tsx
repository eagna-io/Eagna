import React, {FC} from 'react';
import styled from 'styled-components';
import StyledFirebaseAuth from 'react-firebaseui/StyledFirebaseAuth';
import firebase from 'firebase';

import {Market} from 'models/market';
import * as Header from './components/header';
import * as FeaturedMarketComponent from './components/featured_market';
import * as Footer from './components/footer';

interface PcTopPageProps {
  uiConfig: any;
  featuredMarkets: Market[];
}

export const TopPagePc: FC<PcTopPageProps> = ({uiConfig, featuredMarkets}) => {
  return (
    <>
      <Header.Pc />
      <MainSection>
        <MainSectionBgFilter>
          <MainMsg>未来は僕等の手の中</MainMsg>
          <SubMsg>今すぐ予測市場を体験しましょう</SubMsg>
          <StyledFirebaseAuth
            uiConfig={uiConfig}
            firebaseAuth={firebase.auth()}
          />
        </MainSectionBgFilter>
      </MainSection>
      <FeaturedMarketsSection>
        <SectionTitle>注目のマーケット</SectionTitle>
        <FeaturedMarketList>
          {featuredMarkets.map(m => (
            <FeaturedMarketComponent.Pc key={m.id} market={m} />
          ))}
        </FeaturedMarketList>
      </FeaturedMarketsSection>
      <Footer.Pc />
    </>
  );
};

export default TopPagePc;

const MainSection = styled.div`
  height: 787px;
  background-image: url('/img/top/main-bg.png');
  background-position: center;
  background-size: cover;
`;

const MainSectionBgFilter = styled.div`
  width: 100%;
  height: 100%;
  background-color: rgba(255, 255, 255, 0.7);
  padding-top: 200px;
`;

const MainMsg = styled.h2`
  height: 54px;
  width: 100%;
  font-size: 36px;
  font-weight: 400;
  text-align: center;
  margin: 0;
  padding: 0;
`;

const SubMsg = styled.h3`
  height: 45px;
  width: 100%;
  top: 12px;
  font-size: 30px;
  font-weight: 300;
  text-align: center;
  margin: 0;
  margin-top: 12px;
  margin-bottom: 15px;
  padding: 0;
`;

const FeaturedMarketsSection = styled.div`
  width: 100vw;
  padding-top: 64px;
  padding-bottom: 183px;
`;

const FeaturedMarketList = styled.div`
  width: 1010px;
  margin: 0 auto;
`;

const SectionTitle = styled.h3`
  width: 100%;
  height: 54px;
  text-align: center;
  line-height: 54px;
  font-size: 36px;
  font-weight: 100;
  text-align: center;
  margin: 0;
  padding: 0;
`;

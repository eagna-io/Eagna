import React, {FC} from 'react';
import styled from 'styled-components';

import {pc} from 'app/components/responsive';
import SigninComponent from 'app/components/signin';

import Header from './top/components/header';
import ThreeStepsSection from './top/components/three-steps';
import FeaturedMarketComponent from './top/components/featured_market';
import Footer from './top/components/footer';

const TopPage: FC = () => {
  return (
    <>
      <Header />
      <MainSection>
        <MainSectionBgFilter>
          <MainMsg>未来は僕等の手の中</MainMsg>
          <SubMsg>今すぐ予測市場を体験しましょう</SubMsg>
          <SigninComponent redirectUrl="/account" autoRedirect />
          <AnnounceBetaRelease href="https://note.mu/rohan_market/n/n7f8a517c50f6">
            &beta; 版をリリースしました！
          </AnnounceBetaRelease>
        </MainSectionBgFilter>
      </MainSection>
      <ThreeStepsSection />
      <FeaturedMarketComponent />
      <Footer />
    </>
  );
};

export default TopPage;

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

import React, { FC } from "react";
import styled from "styled-components";
import ReactGA from "react-ga";

import { pc } from "app/components/responsive";
import SigninComponent from "app/components/signin";
import TwitterFeed from "app/components/twitter";

import Header from "./components/header";
import ThreeStepsSection from "./components/three-steps";
import FeaturedMarketComponent from "./components/featured_market";
import Footer from "./components/footer";

const TopPage: FC = () => {
  React.useEffect(() => {
    ReactGA.pageview("/");
  }, []);
  return (
    <>
      <Header />
      <MainSection id="main">
        <MainSectionBgFilter>
          <MainMsg>未来は僕等の手の中</MainMsg>
          <SubMsg>今すぐ予測市場を体験しましょう</SubMsg>
          <SubMsg2>どなたでも無料でご利用できます</SubMsg2>
          <SigninComponent redirectUrl="/account" />
          <AnnounceBetaRelease href="https://note.mu/rohan_market/n/n7f8a517c50f6">
            &beta; 版をリリースしました！
          </AnnounceBetaRelease>
        </MainSectionBgFilter>
      </MainSection>
      <ThreeStepsSection />
      <FeaturedMarketComponent />
      <TwitterFeed center />
      <Footer />
    </>
  );
};

export default TopPage;

const MainSection = styled.div`
  height: 448px;
  background-image: url("/img/top/main-bg.jpg");
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
  font-size: 16px;
  font-weight: 300;
  text-align: center;
  line-height: 24px;
  margin: 13px 0 5px 0;
  padding: 0;

  ${pc(`
    height: 45px;
    font-size: 30px;
    line-height: 45px;
  `)}
`;

const SubMsg2 = styled.h3`
  height: 24px;
  width: 100%;
  font-size: 14px;
  font-weight: 300;
  text-align: center;
  line-height: 24px;
  margin: 10px 0 30px 0;
  padding: 0;

  ${pc(`
    height: 45px;
    font-size: 24px;
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

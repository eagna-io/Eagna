import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Contest } from "model/contest";

import { ContestComponent } from "./components/organisms/contest";
import { ReactComponent as LogoWhite } from "./components/atoms/images/PlayPoll_logo_white.svg";
import { ReactComponent as LogoGrad } from "./components/atoms/images/PlayPoll_logo_grad.svg";
import { ReactComponent as FireIcon } from "./components/atoms/images/fire.svg";
import { ReactComponent as TimerIcon } from "./components/atoms/images/clock-w.svg";

interface Props {
  contests: Contest[];
}

export const Page: React.FC<Props> = ({ contests }) => {
  return (
    <Container>
        <Wrapper>
          <Header><LogoW /></Header>
          <Openning>
            <LargeFire />
            <SectionTitle>開催中のコンテスト</SectionTitle>
          </Openning>
          {contests.map(contest => (
            contest.status === "Open" ? <ContestComponent contest={contest}/> : null
          ))}
          <Upcoming>
            <LargeTimer />
            <SectionTitle>開催予定のコンテスト</SectionTitle>
          </Upcoming>
          {contests.map(contest => (
            contest.status === "Upcoming" ? <ContestComponent contest={contest}/> : null
          ))}
        </Wrapper>
        <Footer>
          <LogoG />
          <FooterList>
            <FooterListItem><Link href="/play-guide.pdf" target="_blank">プレイガイド</Link></FooterListItem>
            <FooterListItem><Link href="/terms.pdf" target="_blank">利用規約</Link></FooterListItem>
            <FooterListItem><Link href="/privacy-policy.pdf" target="_blank">プライバシーポリシー</Link></FooterListItem>
            <FooterListItem><Link href="https://www.crop-predictionmarket.com/" target="_blank">運営会社</Link></FooterListItem>
          </FooterList>
          <CopyRights>©2020 crop,Inc.</CopyRights>
        </Footer>
    </Container>
  );
}

const Container = styled.div`
  position: relative;
  width: 100vw;
  background-image: linear-gradient(151deg, ${color.WildWatermelon.hex} 0%, ${color.ToreaBay.hex} 100%);
  user-select: none;
`;

const Wrapper = styled.div`
  position: relative;
  padding: 16px 28px 432px 28px;
`;

const Header = styled.div`
  width: 100%;
  margin-bottom: 16px;
`;

const Openning = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
  margin-bottom: 26px;
`;

const LargeFire = styled(FireIcon)`
  width: 20px;
  height: 20px;
  margin-right: 4px;
`;

const SectionTitle = styled.div`
  font-size: 18px;
  font-weight: 800;
  color: ${color.WhiteBaseColor.hex};
  letter-spacing: 1px;
`;

const Upcoming = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
  margin-bottom: 26px;
`;

const LargeTimer = styled(TimerIcon)`
  width: 18px;
  height: 18px;
  margin-right: 4px;
`;

const LogoW = styled(LogoWhite)`
  width: 136px;
  height: 31px;
  margin-top: 8px;
  margin-right: auto;
`;

const Footer = styled.div`
  position: absolute;
  bottom: 0;
  width: 100%;
  padding: 40px 0 24px 0;
  background: ${color.WhiteBaseColor.rgba(0.9)};
`;

const LogoG = styled(LogoGrad)`
  width: 100%;
  height: 32px;
  text-align: center;
`;

const FooterList = styled.ul`
  width: 200px;
  margin: 60px auto;
  padding: 0;
  list-style: none;
`;

const FooterListItem = styled.li`
  font-size: 16px;
  margin-bottom: 32px;
  text-align: center;
  color: ${color.TextBaseColor.hex};
  &:last-child {
    margin-bottom: 0;
  }
`;

const Link = styled.a`
  text-decoration: none;
`;

const CopyRights = styled.div`
  text-align: center;
  font-size: 12px;
  color: ${color.TextBaseColor.hex};
`;

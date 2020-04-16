import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { ReactComponent as LogoWhite } from "./components/atoms/images/PlayPoll_logo_white.svg";
import { ReactComponent as LogoGrad } from "./components/atoms/images/PlayPoll_logo_grad.svg";

export const Top: React.FC = () => {
  return (
    <Container>
        <Wrapper>
          <Header><LogoW /></Header>
        </Wrapper>
        <Footer>
          <LogoG />
          <FooterList>
            <FooterListItem>プレイガイド</FooterListItem>
            <FooterListItem>利用規約</FooterListItem>
            <FooterListItem>プライバシーポリシー</FooterListItem>
            <FooterListItem>運営会社</FooterListItem>
          </FooterList>
          <CopyRights>©2020 crop,Inc.</CopyRights>
        </Footer>
    </Container>
  );
}

const Container = styled.div`
  position: relative;
  width: 100vw;
  height: 100vh;
  background-image: linear-gradient(151deg, ${color.WildWatermelon.hex} 0%, ${color.ToreaBay.hex} 100%);
  user-select: none;
`;

const Wrapper = styled.div`
  position: relative;
  padding: 16px 28px 387px 28px;
`;

const Header = styled.div`
  width: 100%;
  margin-bottom: 16px;
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

const CopyRights = styled.div`
  text-align: center;
  font-size: 12px;
  color: ${color.TextBaseColor.hex};
`;
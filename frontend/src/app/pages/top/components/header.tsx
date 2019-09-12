import React, { FC } from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import { pc } from "app/components/responsive";
import { NavyBlue } from "app/components/color";

const Header: FC<{}> = React.memo(() => {
  return (
    <Container>
      <Logo src="/img/logo.png" />
      <LoginButton to="/login">ログイン</LoginButton>
      <HeaderLinks>
        <HeaderLink href="https://note.mu/rohan_market/n/n017432cef64f">
          操作説明
        </HeaderLink>
        <HeaderLink href="https://note.mu/rohan_market/n/nba87159eace9">
          Eagnaとは？
        </HeaderLink>
        <HeaderLink href="https://note.mu/rohan_market/n/naed2a34bf6e6">
          予測市場とは？
        </HeaderLink>
      </HeaderLinks>
    </Container>
  );
});

export default Header;

const Container = styled.header`
  position: relative;
  width: 100%;

  ${pc(`
    height: 140px;
  `)}
`;

const Logo = styled.img`
  display: block;
  width: 130px;
  height: 65px;
  margin: 10px 0 0 23px;

  ${pc(`
    position: absolute;
    width: 200px;
    height: 100px;
    left: 50px;
    top: 20px;
  `)}
`;

const HeaderLinks = styled.div`
  display: flex;
  flex-direction: row-reverse;
  justify-content: space-around;
  width: 100%;
  height: 45px;

  ${pc(`
    position: absolute;
    height: 100px;
    width: 630px;
    right: 180px;
    top: 20px;
  `)}
`;

const HeaderLink = styled.a`
  display: inline-block;
  height: 45px;
  width: fit-content;
  font-weight: 300;
  font-size: 13px;
  text-align: center;
  line-height: 45px;

  ${pc(`
    width: fit-content;
    height: 100px;
    line-height: 100px;
    margin-right: 70px;
    font-size: 20px;
  `)}
`;

const LoginButton = styled(Link)`
  display: block;
  position: absolute;
  top: 20px;
  right: 30px;
  width: 70px;
  height: 30px;
  border-radius: 3px;
  border: 1px solid ${NavyBlue.hex};
  font-size: 13px;
  font-weight: 300;
  line-height: 28px;
  text-align: center;

  ${pc(`
    top: 43px;
    right: 70px;
    width: 100px;
    height: 50px;
    font-size: 20px;
    line-height: 50px;
  `)}
`;

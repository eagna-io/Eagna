import React, {FC} from 'react';
import styled from 'styled-components';

import {pc} from 'app/components/responsive';

const Header: FC<{}> = React.memo(() => {
  return (
    <Container>
      <Logo src="/img/logo.png" />
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
  width: 100%;

  ${pc(`
    height: 140px;
  `)}
`;

const Logo = styled.img`
  display: block;
  width: 130px;
  height: 65px;
  margin: 0 auto;

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
  width: 100%;
  height: 45px;

  ${pc(`
    position: absolute;
    height: 100px;
    width: 660px;
    right: 0px;
    top: 20px;
  `)}
`;

const HeaderLink = styled.a`
  display: inline-block;
  height: 45px;
  width: 33.33%;
  font-weight: 300;
  font-size: 13px;
  text-align: center;
  line-height: 45px;

  ${pc(`
    width: 150px;
    height: 100px;
    line-height: 100px;
    margin-right: 70px;
    font-size: 20px;
  `)}
`;

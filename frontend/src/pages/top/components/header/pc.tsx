import React, {FC} from 'react';
import styled from 'styled-components';

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
  height: 140px;
`;

const Logo = styled.img`
  position: absolute;
  width: 200px;
  height: 100px;
  left: 50px;
  top: 20px;
`;

const HeaderLinks = styled.div`
  position: absolute;
  display: flex;
  height: 100px;
  right: 0px;
  top: 20px;
  flex-direction: row-reverse;
`;

const HeaderLink = styled.a`
  height: 100px;
  line-height: 100px;
  margin-right: 70px;
  font-weight: 300;
  font-size: 20px;
`;

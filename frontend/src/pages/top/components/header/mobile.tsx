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
`;

const Logo = styled.img`
  display: block;
  width: 130px;
  height: 65px;
  margin: 0 auto;
`;

const HeaderLinks = styled.div`
  display: flex;
  width: 100%;
  height: 45px;
  justify-content: space-around;
`;

const HeaderLink = styled.a`
  height: 45px;
  width: 33.3%;
  font-weight: 300;
  font-size: 13px;
  text-align: center;
  line-height: 45px;
`;

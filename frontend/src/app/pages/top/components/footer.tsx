import React, {FC} from 'react';
import styled from 'styled-components';
import {pc} from 'app/components/responsive';

const Footer: FC<{}> = React.memo(() => {
  return (
    <Container>
      <Logo src="/img/logo.png" />
      <Copyright>&copy; 2019 Eagna inc</Copyright>
      <Contact>ご質問・お問い合わせ</Contact>
      <Email>info@eagna.io</Email>
    </Container>
  );
});

export default Footer;

const Container = styled.footer`
  position: relative;
  width: 100%;
  height: 90px;
  background-color: #1b384e;

  ${pc(`
    height: 236px;
  `)}
`;

const Logo = styled.img`
  width: 96px;
  height: 48px;
  position: absolute;
  top: 10px;
  left: 20px;

  ${pc(`
    width: 315px;
    height: 158px;
    top: 0px;
    left: 15px;
  `)}
`;

const Copyright = styled.div`
  height: 15px;
  position: absolute;
  top: 62px;
  left: 20px;
  font-size: 9px;
  font-weight: 400;
  color: #5bb192;

  ${pc(`
    height: 30px;
    top: 171px;
    left: 62px;
    font-size: 20px;
  `)}
`;

const Contact = styled.p`
  position: absolute;
  right: 28px;
  top: 28px;
  font-size: 12px;
  font-weight: 400;
  text-align: right;
  color: white;

  ${pc(`
    right: 140px;
    top: 140px;
    font-size: 16px;
  `)}
`;

const Email = styled.p`
  position: absolute;
  right: 28px;
  top: 50px;
  font-size: 11px;
  font-weight: 400;
  text-align: right;
  color: white;

  ${pc(`
    right: 140px;
    top: 180px;
    font-size: 14px;
  `)}
`;

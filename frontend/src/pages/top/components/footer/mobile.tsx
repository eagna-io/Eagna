import React, {FC} from 'react';
import styled from 'styled-components';

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
`;

const Logo = styled.img`
  width: 96px;
  height: 48px;
  position: absolute;
  top: 10px;
  left: 20px;
`;

const Copyright = styled.div`
  height: 15px;
  position: absolute;
  top: 62px;
  left: 20px;
  font-size: 9px;
  font-weight: 400;
  color: #5bb192;
`;

const Contact = styled.p`
  position: absolute;
  right: 28px;
  top: 28px;
  font-size: 12px;
  font-weight: 400;
  text-align: right;
  color: white;
`;

const Email = styled.p`
  position: absolute;
  right: 28px;
  top: 50px;
  font-size: 11px;
  font-weight: 400;
  text-align: right;
  color: white;
`;

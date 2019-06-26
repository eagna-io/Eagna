import React, {FC} from 'react';
import styled from 'styled-components';

const Footer: FC<{}> = React.memo(() => {
  return (
    <Container>
      <Logo src="/img/logo.png" />
      <Copyright>&copy; 2019 Eagna inc</Copyright>
      <Contact>ご質問・お問い合わせ</Contact>
      <Email>marketrohan@gmail.com</Email>
    </Container>
  );
});

export default Footer;

const Container = styled.footer`
  position: relative;
  width: 100%;
  height: 236px;
  background-color: #1b384e;
`;

const Logo = styled.img`
  width: 315px;
  height: 158px;
  position: absolute;
  top: 0px;
  left: 51px;
`;

const Copyright = styled.div`
  height: 30px;
  position: absolute;
  top: 171px;
  left: 62px;
  font-size: 20px;
  font-weight: 400;
  line-height: 30px;
  color: #5bb192;
`;

const Contact = styled.p`
  position: absolute;
  right: 140px;
  top: 140px;
  font-size: 16px;
  font-weight: 400;
  text-align: right;
  color: white;
`;

const Email = styled.p`
  position: absolute;
  right: 140px;
  top: 180px;
  font-size: 14px;
  font-weight: 400;
  text-align: right;
  color: white;
`;

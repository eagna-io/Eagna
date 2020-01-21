import React from "react";
import styled from "styled-components";

const Footer: React.FC = () => {
  return (
    <Container>
      <ContactInfo>
        ご質問・お問い合わせ
        <br />
        info@crop-pm.com
      </ContactInfo>
    </Container>
  );
};

export default Footer;

const Container = styled.div`
  width: 100vw;
  height: 150px;
  position: relative;
  background-color: #1b384e;
`;

const ContactInfo = styled.div`
  position: absolute;
  right: 140px;
  bottom: 50px;
  font-size: 14px;
  font-weight: bold;
  color: white;
`;

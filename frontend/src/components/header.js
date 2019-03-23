import React from 'react';
import styled from 'styled-components';

import { AccountPage, Link } from 'src/router';

export default function Header(props) {
  return (
    <Container className={props.className}>
      <Title>Rohan Market</Title>
      <Profile to={AccountPage()}></Profile>
    </Container>
  );
}

const Container = styled.div`
  width: 100vw;
  height: 80px;
  background-color: #358ED7;
  border-bottom: 1px solid #979797;
`;

const Title = styled.div`
  display: inline-block;
  font-size: 26px;
  font-family: "Americal Typewriter";
  font-weight: bold;
  color: white;
  height: 80px;
  line-height: 80px;
  margin-left: 30px;
`;

const Profile = styled(Link)`
  display: inline-block;
  position: absolute;
  top: 16px;
  right: 20px;
  width: 48px;
  height: 48px;
  background-color: white;
  border-radius: 4px;
`;

import React from 'react';
import styled from 'styled-components';

import { Link } from 'react-router-dom';

export default function Header(props) {
  return (
    <Container className={props.className}>
      <Link to="/"><Logo src="/img/logo.png" /></Link>
      <Profile to="/me"></Profile>
    </Container>
  );
}

const Container = styled.div`
  display: flex;
  width: 100vw;
  height: 80px;
  padding: 0 30px;
  background-color: #358ED7;
  border-bottom: 1px solid #979797;
  justify-content: space-between;
  align-items: center;
`;

const Logo = styled.img`
  display: block;
  height: 80px;
  margin-left: 30px;
`;

const Profile = styled(Link)`
  display: block;
  width: 48px;
  height: 48px;
  background-color: white;
  border-radius: 4px;
`;

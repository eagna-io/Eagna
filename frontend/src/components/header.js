import React from 'react';
import styled from 'styled-components';

import { Link } from 'react-router-dom';

export default function Header(props) {
  return (
    <Container className={props.className}>
      <Link to="/"><Logo src="/img/logo.png" /></Link>
      <Profile to="/me">
        <i className="fas fa-user-circle"></i>
      </Profile>
    </Container>
  );
}

const Container = styled.div`
  display: flex;
  width: 100vw;
  height: 60px;
  padding: 0 30px;
  background-color: #1c384e;
  border-bottom: 1px solid #979797;
  justify-content: space-between;
  align-items: center;
`;

const Logo = styled.img`
  display: block;
  height: 50px;
  margin-left: 30px;
`;

const Profile = styled(Link)`
  display: block;
  width: 40px;
  height: 40px;
  font-size: 30px;
  text-align: center;
  line-height: 40px;
  color: #1c384e;
  background-color: white;
  border-radius: 4px;
`;

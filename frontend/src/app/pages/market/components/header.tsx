import React from "react";
import styled from "styled-components";

interface Props {
    userName: string;
  }

const Header: React.FC<Props> = ({ userName }) => {
  return (
    <Container>
      <Account>{userName}</Account>
      <LiveIcon>{liveIcon}</LiveIcon>
    </Container>
  );
};

export default Header;

const liveIcon = 'LIVE'

const Container = styled.div`
  position: relative;
  background-color: #242423;
  padding: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
`;

const Account = styled.div`
  color: white;
  font-size: 14px;
  font-weight: 500;
`;

const LiveIcon = styled.div`
  color: white;
  background-color: #BB86FC;
  border-radius: 4px;
  padding: 2px 4px;
  font-size: 14px;
  font-weight: 500;
`;

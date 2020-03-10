import React from "react";
import styled from "styled-components";
import { WhiteBaseColor, TextBaseColor, PurpleColor } from "app/components/color";

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
  margin-bottom: 8px;
  display: flex;
  justify-content: space-between;
  align-items: center;
`;

const Account = styled.div`
  color: ${TextBaseColor.hex};
  font-size: 14px;
  font-weight: 500;
`;

const LiveIcon = styled.div`
  color: ${WhiteBaseColor.hex};
  background-color: ${PurpleColor.hex};
  border-radius: 4px;
  padding: 2px 4px;
  font-size: 14px;
  font-weight: 500;
`;

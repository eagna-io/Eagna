import React from "react";
import styled from "styled-components";
import { Color, BackgroundMainColor, PurpleColor} from "app/components/color";

interface Props {
    userName: string;
  }

const Header: React.FC<Props> = ({ userName }) => {
  return (
    <Container bgcolor={BackgroundMainColor}>
      <Account>{userName}</Account>
      <LiveIcon purpleColor={PurpleColor}>{liveIcon}</LiveIcon>
    </Container>
  );
};

export default Header;

const liveIcon = 'LIVE'

const Container = styled("div")<{ bgcolor: Color }>`
  position: relative;
  background-color: ${props => props.bgcolor.hex};
  margin-bottom: 8px;
  display: flex;
  justify-content: space-between;
  align-items: center;
`;

const Account = styled.div`
  color: white;
  font-size: 14px;
  font-weight: 500;
`;

const LiveIcon = styled("div")<{ purpleColor: Color }>`
  color: white;
  background-color: ${props => props.purpleColor.hex};
  border-radius: 4px;
  padding: 2px 4px;
  font-size: 14px;
  font-weight: 500;
`;

import React from "react";
import styled from "styled-components";
import { Color, WhiteBaseColor, BackgroundMainColor, PurpleColor} from "app/components/color";

interface Props {
    userName: string;
  }

const Header: React.FC<Props> = ({ userName }) => {
  return (
    <Container bgcolor={BackgroundMainColor}>
      <Account whiteBaseColor={WhiteBaseColor}>{userName}</Account>
      <LiveIcon purpleColor={PurpleColor} whiteBaseColor={WhiteBaseColor}>{liveIcon}</LiveIcon>
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

const Account = styled("div")<{ whiteBaseColor: Color }>`
  color: ${props => props.whiteBaseColor.hex};
  font-size: 14px;
  font-weight: 500;
`;

const LiveIcon = styled("div")<{ purpleColor: Color, whiteBaseColor: Color }>`
  color: ${props => props.whiteBaseColor.hex};
  background-color: ${props => props.purpleColor.hex};
  border-radius: 4px;
  padding: 2px 4px;
  font-size: 14px;
  font-weight: 500;
`;

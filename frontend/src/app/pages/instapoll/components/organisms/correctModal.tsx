import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  Correct
} from "app/components/color";

export const CorrectModal: React.FC = () => {
  let [ showFlag, setFlag ] = React.useState(true);
  
  // 3秒後モーダルが非表示に
  React.useEffect(() => {
    const timer = setTimeout(() => {
      setFlag(showFlag = false)
    }, 3000);
    return () => clearTimeout(timer);
  }, []);
  
  return (
    <Container show={showFlag}>正解！</Container>
  );
};

const Container = styled.div<{ show: boolean }>`
  display: ${props => props.show ? "block" : "none"}
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit-transform: translateY(-50%) translateX(-50%);
  width: 286px;
  height: 97px;
  background-color: ${Correct.rgba(0.9)};
  font-size: 32px;
  line-height: 97px;
  font-weight: 800;
  letter-spacing: 1.14px;
  color: ${WhiteBaseColor.hex};
  text-align: center;
  z-index: 100;
`;

import React from "react";
import styled from "styled-components";

import {
  WhiteBaseColor,
  Correct,
  MainRed
} from "app/components/color";

interface Props {
  isCorrect: boolean
}

export const ResultModal: React.FC<Props> = ({ isCorrect }) => {
  const [ isShow, setIsShow ] = React.useState(true);
  
  // 3秒後モーダルが非表示に
  React.useEffect(() => {
    const timer = setTimeout(() => {
      setIsShow(false)
    }, 3000);
    return () => clearTimeout(timer);
  }, []);
  
  return (
    <Container isCorrect={isCorrect} show={isShow}>
      { isCorrect ? "正解！" : "残念..." }
    </Container>
  );
};

const Container = styled.div<{ show: boolean, isCorrect: boolean }>`
  position: absolute;
  top: ${props => props.show ? "50vh" : "-50vh"};
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit-transform: translateY(-50%) translateX(-50%);
  width: 286px;
  height: 97px;
  background-color: ${props => props.isCorrect ? Correct.rgba(0.9) : MainRed.rgba(0.9)};
  font-size: 32px;
  line-height: 97px;
  font-weight: 800;
  letter-spacing: 1.14px;
  color: ${WhiteBaseColor.hex};
  text-align: center;
  z-index: 100;
  opacity: ${props => props.show ? "1" : "0"};
  transition: 1s;
`;

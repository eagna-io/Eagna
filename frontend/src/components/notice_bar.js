import React, { useState, useEffect } from 'react';
import styled from 'styled-components';


export default function NoticeBar(props) {
  const msg = props.children;
  const nonce = props.nonce;
  const [dissappearing, setDisappearing] = useState(true);

  useEffect(() => {
    if (msg) {
      setDisappearing(false);
      const timerId = window.setTimeout(() => setDisappearing(true), 4000);
      return () => {
        window.clearTimeout(timerId);
      }
    }
  }, [msg, nonce]);


  return (
    <Container dissappearing={dissappearing}>
      <Msg>{msg}</Msg>
    </Container>
  );
}

const Container = styled.div`
  position: absolute;
  z-index: 5;
  top: ${props => props.dissappearing ? "-50px" : "0px"};
  left: calc((100vw - 500px) / 2);
  width: 500px;
  height: 50px;
  background-color: #4A90E2;
  border-radius: 0px 0px 4px 4px;
  transition: all 1000ms 0s ease;
  box-shadow: 0px 4px 4px 0px rgba(0,0,0,0.5);
`;

const Msg = styled.h5`
  width: 100%;
  height: 50px;
  line-height: 50px;
  text-align: center;
  font-size: 20px;
  color: white;
  margin: 0;
`;

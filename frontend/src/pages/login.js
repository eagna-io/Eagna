import React, { useState, useContext } from 'react';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import { AccessTokenContext } from 'src/context';
import { getAccessToken, LoginFailedError, NetworkError } from 'src/api';

export default function LoginPage() {
  const {token, setToken} = useContext(AccessTokenContext);
  const [emailInput, setEmailInput] = useState("");
  const [passInput, setPassInput] = useState("");
  const [requesting, setRequesting] = useState(false);
  const [errMsg, setErrMsg] = useState(null);

  // すでにアクセストークンを取得している場合は
  // アカウントページにリダイレクトする
  if (token) {
    return <Redirect to="/me" />
  }

  // TODO : Loading の表示方法
  if (requesting) {
    return <h3>Requesting...</h3>
  }

  const requestLogin = () => {
    setRequesting(true);
    setErrMsg(null);
    getAccessToken(emailInput, passInput)
      .then(accessToken => setToken(accessToken))
      .catch(err => {
        switch(err) {
          case LoginFailedError:
            setErrMsg("Email or Password is incorrect");
            break;
          case NetworkError:
          default:
            setErrMsg("Network error is detected");
        }
      });
  };

  return (
    <Body>
      <Container>
        <Title>ROHAN MARKET</Title>
        { errMsg ? <h3>{errMsg}</h3> : null /* TODO : css style */ }
        <Input
          type="text"
          placeholder="Email"
          value={emailInput}
          onChange={e => setEmailInput(e.target.value)} 
          onKeyPress={e => isPressEnter(e) && requestLogin()} />
        <Input
          type="password"
          placeholder="Password"
          value={passInput}
          onChange={e => setPassInput(e.target.value)}
          onKeyPress={e => isPressEnter(e) && requestLogin()} />
        <SubmitButton
          onClick={e => {
            e.preventDefault();
            requestLogin()
          }}>
            Login
        </SubmitButton>
      </Container>
    </Body>
  );
}

function isPressEnter(e) {
  if (e.which === 13) {
    e.preventDefault();
    return true;
  } else {
    return false;
  }
}

const Body = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: #F5F8FD;
  position: absolute;
  top: 0px;
  left: 0px;
`;

const Container = styled.div`
  margin-top: 30vh;
  margin-left: 30vw;
  width: 40vw;
`;

const Title = styled.div`
  color: #84B6F9;
  margin-left: 30vw;
  width: 40vw;
`;

const Input = styled.input`
  width: 100%;
  height: 40px;
  margin-top: 20px;
  font-size: 20px;
  padding-left: 10px;
`;

const SubmitButton = styled.button`
  width: 100%;
  height: 50px;
  margin-top: 20px;
  background-color: #84B6F9;
  color: white;
  font-size: 20px;
  font-family: "Arial";
`;

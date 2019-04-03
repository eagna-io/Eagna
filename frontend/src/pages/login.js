import React, { useState, useContext, useEffect } from 'react';
import styled from 'styled-components';

import { AccessTokenContext } from 'src/context';
import { createAccessToken, getAccessToken,
  LoginFailedError, NetworkError, InvalidAccessTokenError } from 'src/api';
import NoticeBar from 'src/components/notice_bar';
import Loading from 'src/components/loading';

export default function LoginPage(props) {
  const history = props.history;
  const {token, setToken} = useContext(AccessTokenContext);
  const [emailInput, setEmailInput] = useState("");
  const [passInput, setPassInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [[errMsg, errNonce], setErr] = useState([null, null]);

  // すでのアクセストークンを持っている場合はそれが
  // 有効なものかチェックする
  useEffect(() => {
    if (token) {
      getAccessToken(token)
        .then(info => {
          history.push("/me");
        })
        .catch(err => {
          switch(err) {
            case InvalidAccessTokenError:
              setToken(null);
              break;
          }
        });
    }
  }, [token]);

  const requestLogin = () => {
    setLoading(true);
    setErr([null, null]);
    createAccessToken(emailInput, passInput)
      .then(accessToken => {
        history.push("/me");
        setToken(accessToken);
      })
      .catch(err => {
        setLoading(false);
        switch(err) {
          case LoginFailedError:
            setErr(["Email or Password is incorrect", Date.now()]);
            break;
          case NetworkError:
            setErr(["Network error is detected", Date.now()]);
            break;
          default:
            setErr("Sorry. Server error is detected. Please try again later");
            console.error(err);
            break;
        }
      });
  };

  return (
  <>
    <Loading loading={loading} />
    <NoticeBar nonce={errNonce}>{errMsg}</NoticeBar>
    <Body>
      <Container>
        <Title>ROHAN MARKET</Title>
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
    </>
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

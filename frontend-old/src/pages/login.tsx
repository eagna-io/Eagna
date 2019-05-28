import React, { useState, useContext, useEffect } from 'react';
import styled from 'styled-components';

import { AccessTokenContext } from 'context/access_token';
import { createAccessToken, getAccessToken,
  LoginFailedError, NetworkError, InvalidAccessTokenError } from 'api/api';
import NoticeBar from 'components/notice_bar';
import Loading from 'components/loading';

export default function LoginPage(props) {
  const history = props.history;
  const {accessToken, setAccessToken} = useContext(AccessTokenContext);
  const [emailInput, setEmailInput] = useState("");
  const [passInput, setPassInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [[errMsg, errNonce], setErr] = useState([null, null]);

  // すでのアクセストークンを持っている場合はそれが
  // 有効なものかチェックする
  useEffect(() => {
    if (accessToken) {
      getAccessToken(accessToken)
        .then(info => {
          history.push("/me");
        })
        .catch(err => {
          switch(err) {
            case InvalidAccessTokenError:
              setAccessToken(null);
              break;
          }
        });
    }
  }, [accessToken]);

  const requestLogin = () => {
    setLoading(true);
    setErr([null, null]);
    createAccessToken(emailInput, passInput)
      .then(accessToken => {
        history.push("/me");
        setAccessToken(accessToken);
      })
      .catch(err => {
        setLoading(false);
        switch(err) {
          case LoginFailedError:
            setErr(["EmailかPasswordが違います", Date.now()]);
            break;
          case NetworkError:
            setErr(["ネットワークエラー", Date.now()]);
            break;
          default:
            setErr("サーバーエラーが検知されました");
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
        <Logo src="/img/logo-big.png" />
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
  position: absolute;
  top: 0px;
  left: 0px;
`;

const Container = styled.div`
  margin-top: 30vh;
  margin-left: 30vw;
  width: 40vw;
`;

const Logo = styled.img`
  display: block;
  width: 40%;
  margin: 0 auto;
  margin-bottom: 50px;
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
  background-color: #1c384e;
  color: white;
  font-size: 20px;
  font-family: "Arial";
`;
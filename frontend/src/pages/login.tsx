import React, {FC, useEffect} from 'react';
import styled from 'styled-components';
import {History, LocationDescriptor} from 'history';
import StyledFirebaseAuth from 'react-firebaseui/StyledFirebaseAuth';
import firebase from 'firebase';

import User from 'models/user';

interface LoginPageProps {
  history: History<{redirect?: LocationDescriptor}>;
  user: User | null;
}

const LoginPage: FC<LoginPageProps> = ({history, user}) => {
  useEffect(() => {
    if (user != null) {
      const redirectLocation =
        history.location.state && history.location.state.redirect;
      if (!redirectLocation) {
        history.push('/me');
      } else if (typeof redirectLocation === 'string') {
        history.push(redirectLocation);
      } else {
        history.push(redirectLocation);
      }
    }
  }, [user]);

  // 認証が成功した後のフローは、app.tsxに戻る
  const uiConfig = {
    signInSuccessUrl: '/me',
    signInOptions: [
      {
        provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
        scopes: ['https://www.googleapis.com/auth/userinfo.email'],
        customParameters: {
          prompt: 'select_account',
        },
      },
      {
        provider: firebase.auth.FacebookAuthProvider.PROVIDER_ID,
        scopes: ['email'],
      },
      {
        provider: firebase.auth.GithubAuthProvider.PROVIDER_ID,
        scopes: ['user:email'],
      },
      {
        provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
        requireDisplayName: true,
      },
    ],
  };

  return (
    <>
      <Body>
        <Container>
          <Logo src="/img/logo-big.png" />
          <StyledFirebaseAuth
            uiConfig={uiConfig}
            firebaseAuth={firebase.auth()}
          />
          <div id="firebaseui-auth-container" />
        </Container>
      </Body>
    </>
  );
};

export default LoginPage;

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

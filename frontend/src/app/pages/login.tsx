import React, {FC} from 'react';
import styled from 'styled-components';
import {withRouter} from 'react-router-dom';
import {History} from 'history';
import StyledFirebaseAuth from 'react-firebaseui/StyledFirebaseAuth';
import * as firebase from 'firebase/app';
import 'firebase/auth';

import {LoginStatus, withUser} from 'app/components/user';

interface LoginPageProps {
  history: History;
  user: LoginStatus;
}

const LoginPage: FC<LoginPageProps> = ({history, user}) => {
  const redirectUrl = getRedirectUrl(history);
  console.log(redirectUrl);

  /*
  useEffect(() => {
    if (user instanceof User) {
      history.push(redirectUrl);
    }
  }, [user, history]);
   */

  const config = createAuthConfig(redirectUrl);

  // 認証が成功した後のフローは、app.tsxに戻る
  return (
    <>
      <Body>
        <Container>
          <Logo src="/img/logo-big.png" />
          <StyledFirebaseAuth
            uiConfig={config}
            firebaseAuth={firebase.auth()}
          />
        </Container>
      </Body>
    </>
  );
};

export default withRouter(withUser(LoginPage));

function getRedirectUrl(history: History): string {
  if (
    history.location.state &&
    history.location.state.redirect &&
    typeof history.location.state.redirect === 'string'
  ) {
    return history.location.state.redirect;
  } else {
    return '/account';
  }
}

function createAuthConfig(redirect: string): any {
  return {
    signInSuccessUrl: redirect,
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
`;

const Logo = styled.img`
  display: block;
  width: 60%;
  max-width: 300px;
  margin: 0 auto;
  margin-bottom: 50px;
`;

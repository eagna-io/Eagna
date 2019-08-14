import React, {FC} from 'react';
import styled from 'styled-components';
import {withRouter} from 'react-router-dom';
import {History} from 'history';

import SigninComponent from 'app/components/signin';

interface LoginPageProps {
  history: History;
}

const LoginPage: FC<LoginPageProps> = ({history}) => {
  const redirectUrl = getRedirectUrl(history);

  return (
    <>
      <Body>
        <Container>
          <Logo src="/img/logo-big.png" />
          <SigninComponent redirectUrl={redirectUrl} autoRedirect />
        </Container>
      </Body>
    </>
  );
};

export default withRouter(LoginPage);

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

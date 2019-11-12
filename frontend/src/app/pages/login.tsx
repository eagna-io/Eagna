import React, { FC } from "react";
import styled from "styled-components";
import { withRouter } from "react-router-dom";
import { History } from "history";
import ReactGA from "react-ga";

interface LoginPageProps {
  history: History;
}

const LoginPage: FC<LoginPageProps> = ({ history }) => {
  React.useEffect(() => {
    ReactGA.pageview("/login");
  }, []);
  const redirectUrl = getRedirectUrl(history);

  return (
    <Container>
      <Logo src="/img/logo-big.png" />
    </Container>
  );
};

export default withRouter(LoginPage);

function getRedirectUrl(history: History): string {
  if (
    history.location.state &&
    history.location.state.redirect &&
    typeof history.location.state.redirect === "string"
  ) {
    return history.location.state.redirect;
  } else {
    return "/account";
  }
}

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

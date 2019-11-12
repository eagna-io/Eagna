import React from "react";
import styled from "styled-components";
import ReactGA from "react-ga";

import SigninForm from "app/components/signin";

export default () => {
  React.useEffect(() => {
    ReactGA.pageview("/login");
  }, []);

  return (
    <Container>
      <Logo src="/img/logo-big.png" />
      <SigninForm />
    </Container>
  );
};

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

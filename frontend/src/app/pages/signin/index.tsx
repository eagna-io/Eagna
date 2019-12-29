import React, { FC, useState, useEffect } from "react";
import styled from "styled-components";
import { withRouter } from "react-router-dom";
import { History } from "history";
import ReactGA from "react-ga";
import { useSelector } from "react-redux";

import { Market, MarketStatus, MarketRepository } from "models/market";
import { User } from "models/user";
import { RootState } from "app/redux";
import { MinPcWidth } from "app/components/responsive";
import Header from "app/components/header";

interface Props {
  history: History;
}

const SigninPageWrapper: FC<Props> = ({ history }) => {
  const user = useSelector((state: RootState) => state.user.user);

  useEffect(() => {
    ReactGA.pageview("/account");
  }, []);

  useEffect(() => {
    if (user) {
      history.push("/account");
    }
  });

  return <SigninPage />;
};

export default withRouter(SigninPageWrapper);

const SigninPage: FC = () => {
  return (
    <>
      <HeaderLogo src="/img/logo.png" />
    </>
  );
};

const HeaderLogo = styled.img`
  width: 114px;
  height: 40px;
  margin: 0 auto auto 0;
`;

const Container = styled.div`
  width: 95%;
  max-width: ${MinPcWidth}px;
  margin: 0 auto;
  padding: 50px 0;
`;

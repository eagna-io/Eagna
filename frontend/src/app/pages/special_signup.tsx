import React, { FC } from "react";
import styled from "styled-components";
import { History, LocationDescriptor } from "history";
import StyledFirebaseAuth from "react-firebaseui/StyledFirebaseAuth";
import * as firebase from "firebase/app";
import "firebase/auth";

import { User } from "models/user";
import Header from "app/components/header";

interface Props {
  history: History<{ redirect?: LocationDescriptor }>;
  user: User | null;
}

const SpecialSignupPage: FC<Props> = ({ history, user }) => {
  const uiConfig = {
    callbacks: {
      signInSuccessWithAuthResult: (result: firebase.auth.UserCredential) => {
        if (result.additionalUserInfo && result.additionalUserInfo.isNewUser) {
          countConversion().then(() => history.push("/me"));
          return false;
        }
        return true;
      }
    },
    signInSuccessUrl: "/me",
    signInFlow: "popup",
    signInOptions: [
      {
        provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
        scopes: ["https://www.googleapis.com/auth/userinfo.email"],
        customParameters: {
          prompt: "select_account"
        }
      },
      {
        provider: firebase.auth.FacebookAuthProvider.PROVIDER_ID,
        scopes: ["email"]
      },
      {
        provider: firebase.auth.GithubAuthProvider.PROVIDER_ID,
        scopes: ["user:email"]
      },
      {
        provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
        requireDisplayName: true
      }
    ]
  };

  return (
    <>
      <Header />
      <Container>
        <LogoContainer>
          <Logo src="/img/logo-big.png" />
          <With>with</With>
          <Logo src="/img/special-signup-logo.png" />
        </LogoContainer>
        <Desc>
          「有益なスタートアップ関連情報まとめ」からEagnaをお知りになった方はこちらのページからご登録ください。
        </Desc>
        <StyledFirebaseAuth
          uiConfig={uiConfig}
          firebaseAuth={firebase.auth()}
        />
      </Container>
    </>
  );
};

export default SpecialSignupPage;

function countConversion(): Promise<Response> {
  const RapidApiKey = process.env.REACT_APP_RAPID_API_KEY;
  if (RapidApiKey === undefined) {
    throw new Error("REACT_APP_RAPID_API_KEY is not defined");
  }
  const headers = new Headers();
  headers.append("Content-Type", "application/json");
  headers.append("X-RapidAPI-Host", "nanosdk-counters-v1.p.rapidapi.com");
  headers.append("X-RapidAPI-Key", RapidApiKey);

  return fetch(
    "https://nanosdk-counters-v1.p.rapidapi.com/counters/special-signup",
    {
      method: "PUT",
      headers
    }
  );
}

const Container = styled.div`
  margin-top: 10vh;

  @media (min-width: 980px) {
    margin-top: 30vh;
  }
`;

const LogoContainer = styled.div`
  width: 60%;
  margin: 0 auto;
  margin-bottom: 20px;

  @media (min-width: 980px) {
    width: 700px;
  }
`;

const Logo = styled.img`
  display: inline-block;
  width: 100%;
  margin-bottom: 10px;

  @media (min-width: 980px) {
    width: 300px;
    vertical-align: middle;
  }
`;

const With = styled.span`
  display: inline-block;
  width: 100%;
  margin-bottom: 10px;
  font-size: 18px;
  font-weight: bold;
  text-align: center;

  @media (min-width: 980px) {
    width: 100px;
    vertical-align: middle;
  }
`;

const Desc = styled.p`
  width: 80%;
  margin: 0px auto 30px auto;
  font-size: 12px;

  @media (min-width: 980px) {
    width: 750px;
    font-size: 14px;
    margin-bottom: 50px;
  }
`;

import React from "react";
import StyledFirebaseAuth from "react-firebaseui/StyledFirebaseAuth";
import * as firebase from "firebase/app";
import "firebase/auth";
import { RouteComponentProps, withRouter } from "react-router-dom";
import { useSelector } from "react-redux";

import { User } from "models/user";
import { RootState } from "app/redux";

interface Props {
  redirectUrl: string;
  autoRedirect?: boolean;
}

const SigninComponent: React.FC<Props & RouteComponentProps> = ({
  redirectUrl,
  autoRedirect,
  history
}) => {
  const user = useSelector((state: RootState) => state.user.user);

  React.useEffect(() => {
    if (autoRedirect) {
      if (user instanceof User) {
        history.push(redirectUrl);
      }
    }
  }, [user, redirectUrl, autoRedirect, history]);

  return (
    <StyledFirebaseAuth
      uiConfig={config(redirectUrl)}
      firebaseAuth={firebase.auth()}
    />
  );
};

export default withRouter(SigninComponent);

function config(redirect: string): any {
  return {
    signInSuccessUrl: redirect,
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
        provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
        requireDisplayName: true
      }
    ]
  };
}

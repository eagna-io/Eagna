import React from 'react';
import StyledFirebaseAuth from 'react-firebaseui/StyledFirebaseAuth';
import * as firebase from 'firebase/app';
import 'firebase/auth';
import {RouteComponentProps, withRouter} from 'react-router-dom';

import {User} from 'models/user';
import {UserProps, withUser} from 'app/components/user';

interface Props {
  redirectUrl: string;
  autoRedirect?: boolean;
}

const SigninComponent: React.FC<Props & UserProps & RouteComponentProps> = ({
  redirectUrl,
  autoRedirect,
  user,
  history,
}) => {
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

export default withRouter(withUser(SigninComponent));

function config(redirect: string): any {
  return {
    signInSuccessUrl: redirect,
    signInFlow: 'popup',
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

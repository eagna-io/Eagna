import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {withRouter} from 'react-router-dom';
import {History} from 'history';

import {Market} from 'models/market';
import {User} from 'models/user';
import {getMarkets, getMyMarkets} from 'api/market';
import {MinPcWidth} from 'app/components/responsive';
import Header from 'app/components/header';
import {withUser, LoginStatus} from 'app/components/user';

import ProfileComponent from './account/components/profile';
import MarketsComponent from './account/components/markets';

interface AccountPageProps {
  history: History;
  user: LoginStatus;
}

const AccountPageWrapper: FC<AccountPageProps> = ({history, user}) => {
  useEffect(() => {
    if (user === null) {
      history.push('/login');
    }
  });

  if (user === null || user === 'Checking') {
    return <h2>Loading...</h2>;
  } else {
    return <AccountPage user={user} />;
  }
};

export default withRouter(withUser(AccountPageWrapper));

const AccountPage: FC<{user: User}> = ({user}) => {
  const [participatedMarkets, setParticipatedMarkets] = useState<Market[]>([]);
  const [featuredMarkets, setFeaturedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    user.getAccessToken().then(accessToken => {
      if (accessToken === null) {
        // User was already logged out
        // AccountPageWrapper から /login にリダイレクトされるはず
      } else {
        getMyMarkets(accessToken).then(markets => {
          if (markets === 'Unauthorized') {
            setParticipatedMarkets([]);
          } else {
            setParticipatedMarkets(markets);
          }
        });
        getMarkets(['Upcoming', 'Open']).then(markets => {
          setFeaturedMarkets(markets);
        });
      }
    });
  }, [user]);

  return (
    <>
      <Header />
      <Container>
        <ProfileComponent user={user} />
        <MarketsComponent
          title="参加しているマーケット"
          markets={participatedMarkets}
        />
        <MarketsComponent
          title="注目のマーケット"
          markets={featuredMarkets}
        />
      </Container>
    </>
  );
};

const Container = styled.div`
  width: 95%;
  max-width: ${MinPcWidth}px;
  margin: 0 auto;
  padding-top: 50px;
`;

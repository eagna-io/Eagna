import React, {FC} from 'react';
import styled from 'styled-components';
import {History} from 'history';

import {User} from 'models/user';
import {Market} from 'models/market';
import * as Header from 'components/header';
import * as ProfileComponent from './components/profile';
import * as MarketsComponent from './components/markets';

interface AccountPageProps {
  history: History;
  user: User;
  participatedMarkets: Market[];
  featuredMarkets: Market[];
}

const Mobile: FC<AccountPageProps> = ({
  history,
  user,
  participatedMarkets,
  featuredMarkets,
}) => {
  return (
    <>
      <Header.Mobile history={history} user={user} />
      <Container>
        <ComponentContainer>
          <ProfileComponent.Mobile me={user} />
        </ComponentContainer>
        <ComponentContainer>
          <MarketsComponent.Mobile
            title="参加しているマーケット"
            markets={participatedMarkets}
          />
        </ComponentContainer>
        <ComponentContainer>
          <MarketsComponent.Mobile
            title="注目のマーケット"
            markets={featuredMarkets}
          />
        </ComponentContainer>
      </Container>
    </>
  );
};

export default Mobile;

const Container = styled.div`
  width: 90%;
  margin: 0 auto;
  padding: 50px 0;
`;

const ComponentContainer = styled.div`
  margin-top: 70px;

  &:first-of-type {
    margin-top: 0px;
  }
`;

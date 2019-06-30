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

const Pc: FC<AccountPageProps> = ({
  history,
  user,
  participatedMarkets,
  featuredMarkets,
}) => {
  return (
    <>
      <Header.Pc history={history} user={user} />
      <Container>
        <ProfileContainer>
          <ProfileComponent.Pc me={user} />
        </ProfileContainer>
        <MarketsComponentsContainer>
          <MarketsComponent.Pc
            title="参加しているマーケット"
            markets={participatedMarkets}
          />
          <FeaturedMarketsComponentContainer>
            <MarketsComponent.Pc
              title="注目のマーケット"
              markets={featuredMarkets}
            />
          </FeaturedMarketsComponentContainer>
        </MarketsComponentsContainer>
      </Container>
    </>
  );
};

export default Pc;

const Container = styled.div`
  width: 980px;
  margin: 0 auto;
  padding-top: 50px;
`;

const ProfileContainer = styled.div`
  display: inline-block;
  width: 330px;
`;

const MarketsComponentsContainer = styled.div`
  display: inline-block;
  width: 600px;
  margin-left: 50px;
  vertical-align: top;
`;

const FeaturedMarketsComponentContainer = styled.div`
  display: block;
  margin-top: 70px;
`;

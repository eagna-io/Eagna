import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';

import {getMarkets, resolveMarket} from 'api/market';
import {User} from 'models/user';
import {Market} from 'models/market';
import {withUser, LoginStatus} from 'app/components/user';
import NotFoundPage from 'app/pages/not_found';

const AdminResolveMarketOrNotFound: FC<{user: LoginStatus}> = ({user}) => {
  if (user instanceof User && user.isAdmin) {
    return <ResolveMarketPage user={user} />;
  } else {
    return <NotFoundPage />;
  }
};

export default withUser<{}>(AdminResolveMarketOrNotFound);

const ResolveMarketPage: FC<{user: User}> = ({user}) => {
  const [closedMarkets, setClosedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    getMarkets(['Closed']).then(setClosedMarkets);
  }, []);

  return (
    <Container>
      <h2>Resolve Market</h2>
      <hr />
      {closedMarkets.map(market => (
        <ResolveMarketComponent
          key={market.id}
          market={market}
          user={user}
          onResolved={() =>
            setClosedMarkets(closedMarkets.filter(m => m.id !== market.id))
          }
        />
      ))}
    </Container>
  );
};

const Container = styled.div`
  width: 980px;
  margin: 0 auto;
  padding: 30px;
`;

interface ResolveMarketComponentProps {
  market: Market;
  onResolved: () => void;
  user: User;
}

const ResolveMarketComponent: FC<ResolveMarketComponentProps> = ({
  market,
  onResolved,
  user,
}) => {
  return (
    <ResolveMarketContainer>
      <Title>{market.attrs.title}</Title>
      <TokenList>
        {market.attrs.tokens.map(token => (
          <TokenContainer key={token.name}>
            <TokenName>{token.name}</TokenName>
            <ResolveButton
              onClick={() => {
                user.getAccessToken().then(accessToken => {
                  if (accessToken === null) {
                    alert('ログインセッションが切れました');
                  } else {
                    resolveMarket(market.id, token.name, accessToken).then(
                      () => {
                        onResolved();
                      },
                    );
                  }
                });
              }}>
              Resolve
            </ResolveButton>
          </TokenContainer>
        ))}
      </TokenList>
    </ResolveMarketContainer>
  );
};

const ResolveMarketContainer = styled.div`
  padding: 30px;
  border-bottom: 1px solid gray;
`;

const Title = styled.h3`
  display: inline-block;
  width: 350px;
  vertical-align: top;
  overflow-wrap: break-word;
  margin-right: 110px;
`;

const TokenList = styled.div`
  display: inline-block;
  width: 400px;
  vertical-align: top;
`;

const TokenContainer = styled.div`
  width: 100%;
`;

const TokenName = styled.h4`
  display: inline-block;
  width: 300px;
`;

const ResolveButton = styled.button`
  display: inline-block;
  width: 100px;
`;

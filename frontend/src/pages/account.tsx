import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';
import {History} from 'history';

import Header from 'components/header';
import * as table from 'components/table';
import User from 'models/user';
import {Market} from 'models/market';
import {getMyMarkets} from 'api/user';

interface AccountPageProps {
  history: History;
  user: User | null;
}

const AccountPage: FC<AccountPageProps> = ({history, user}) => {
  const [markets, setMarkets] = useState<Market[]>([]);

  useEffect(() => {
    if (user == null) {
      history.push('/login', {redirect: '/me'});
      return;
    } else {
      getMyMarkets(user.accessToken).then(ms => setMarkets(ms));
    }
  }, [user]);

  const Page = styled.div`
    width: 100vw;
    background-color: white;
  `;

  const Container = styled.div`
    width: 90%;
    max-width: 980px;
    margin: 0 auto;
    margin-top: 50px;
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: flex-start;
  `;

  return (
    <>
      <Page>
        <Header history={history} user={user} />
        <Container>
          {user ? <Profile me={user} /> : null}
          <Markets markets={markets} />
        </Container>
      </Page>
    </>
  );
};

export default AccountPage;

const Profile: FC<{me: User}> = ({me}) => {
  const Table = styled(table.Table)`
    max-width: 300px;
    margin-top: 50px;
  `;

  const LeftItem = styled.div`
    text-align: left;
    font-size: 14px;
  `;

  const RightItem = styled.div`
    width: 100%;
    text-align: right;
    font-size: 14px;
    font-weight: bold;
    text-align: right;
  `;

  return (
    <Table>
      <table.Header>
        <table.Cell>Profile</table.Cell>
      </table.Header>
      <table.Body>
        <table.Row>
          <table.Cell3>
            <LeftItem>Name</LeftItem>
          </table.Cell3>
          <table.Cell7>
            <RightItem>{me.name}</RightItem>
          </table.Cell7>
        </table.Row>
        <table.Row>
          <table.Cell3>
            <LeftItem>Email</LeftItem>
          </table.Cell3>
          <table.Cell7>
            <RightItem>{me.email}</RightItem>
          </table.Cell7>
        </table.Row>
      </table.Body>
    </Table>
  );
};

const Markets: FC<{markets: Market[]}> = ({markets}) => {
  const Table = styled(table.Table)`
    width: 600px;
    margin-top: 50px;
  `;

  const MarketStatus = styled.div`
    font-size: 14px;
  `;

  const MarketTitle = styled(Link)`
    font-size: 16px;
    font-weight: bold;
  `;

  return (
    <Table>
      <table.Header>
        <table.Cell>Markets</table.Cell>
      </table.Header>
      <table.Body>
        {markets.map(market => (
          <table.Row key={market.id} striped={true}>
            <table.Cell3>
              <MarketStatus>{market.status}</MarketStatus>
            </table.Cell3>
            <table.Cell7>
              <MarketTitle to={`/market/${market.id}`}>
                {market.title}
              </MarketTitle>
            </table.Cell7>
          </table.Row>
        ))}
      </table.Body>
    </Table>
  );
};

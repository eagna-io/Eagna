import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';
import {History} from 'history';

import Header from 'components/header';
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
  const Container = styled.div`
    width: 100%;
    max-width: 300px;
    margin-top: 50px;
    border: 1px solid #d1d5da;
    border-radius: 4px;
  `;

  const ContainerHeader = styled.h3`
    width: 100%;
    height: 40px;
    background-color: #f6f8fa;
    margin: 0;
    padding-left: 20px;
    font-size: 14px;
    color: #586069;
    line-height: 40px;
    border-bottom: 1px solid #d1d5da;
    text-align: left;
  `;

  const Items = styled.table`
    width: 100%;
    table-layout: fixed;
    border-spacing: 0;
    border-collapse: collapse;
    padding: 0;
    margin: 0;
  `;

  const Item = styled.tr`
    width: 100%;
  `;

  const ItemKey = styled.td`
    display: inline-block;
    text-align: left;
    font-size: 14px;
    width: 30%;
    padding: 10px 0px;
    padding-left: 20px;
  `;

  const ItemVal = styled.td`
    display: inline-block;
    text-align: right;
    font-size: 14px;
    font-weight: bold;
    width: 70%;
    padding: 10px 0px;
    padding-right: 20px;
  `;

  return (
    <Container>
      <ContainerHeader>Profile</ContainerHeader>
      <Items>
        <tbody>
          <Item>
            <ItemKey>Name</ItemKey>
            <ItemVal>{me.name}</ItemVal>
          </Item>
          <Item>
            <ItemKey>Email</ItemKey>
            <ItemVal>{me.email}</ItemVal>
          </Item>
        </tbody>
      </Items>
    </Container>
  );
};

const Markets: FC<{markets: Market[]}> = ({markets}) => {
  // const markets = me.markets.sort(sortMarket);
  const Container = styled.div`
    width: 100%;
    max-width: 600px;
    margin-top: 50px;
    border: 1px solid #d1d5da;
    border-radius: 4px;
  `;

  const ContainerHeader = styled.h3`
    width: 100%;
    height: 40px;
    font-size: 14px;
    color: #586069;
    background-color: #f6f8fa;
    line-height: 40px;
    padding-left: 20px;
    margin: 0px;
    border-bottom: 1px solid #d1d5da;
  `;

  const Items = styled.table`
    width: 100%;
    table-layout: fixed;
    border-spacing: 0;
    border-collapse: collapse;
    padding: 0;
    margin: 0;
  `;

  const Item = styled.tr`
    background-color: white;

    &:nth-child(even) {
      background-color: #f9f9f9;
    }
  `;

  const ItemStatus = styled.td`
    display: inline-block;
    text-align: left;
    font-size: 14px;
    width: 20%;
    padding: 20px;
  `;

  const ItemTitle = styled(Link)`
    display: inline-block;
    text-align: left;
    font-size: 14px;
    width: 80%;
  `;
  return (
    <Container>
      <ContainerHeader>Markets</ContainerHeader>
      <Items>
        <tbody>
          {markets.map(market => (
            <Item key={market.id}>
              <ItemStatus>{market.status}</ItemStatus>
              <td>
                <ItemTitle to={`/market/${market.id}`}>
                  {market.title}
                </ItemTitle>
              </td>
            </Item>
          ))}
        </tbody>
      </Items>
    </Container>
  );
};

/*
function sortMarket(a, b) {
  const statusScore = status => {
    switch (status) {
      case 'open':
        return 0;
      case 'preparing':
        return 1;
      case 'closed':
        return 2;
      case 'settled':
        return 3;
    }
  };
  const statusDiff = statusScore(a.status) - statusScore(b.status);
  if (statusDiff === 0) {
    return a.id - b.id;
  } else {
    return statusDiff;
  }
}
*/

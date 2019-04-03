import React, { useState, useEffect, useContext } from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';

import { getMe, InvalidAccessTokenError, NetworkError } from 'src/api';
import { AccessTokenContext } from 'src/context';
import NoticeBar from 'src/components/notice_bar';
import Loading from 'src/components/loading';
import Header from 'src/components/header';

export default function AccountPage(props) {
  const history = props.history;
  const {token, setToken} = useContext(AccessTokenContext);
  const [me, setMe] = useState(null);
  const [loading, setLoading] = useState(false);
  const [[errMsg, errNonce], setErr] = useState([null, null]);

  useEffect(() => {
    if (!token) {
      history.push("/login");
      return;
    }
    setLoading(true);
    getMe(token)
      .then(me => {
        setMe(me);
        setLoading(false);
      })
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setToken(null);
            history.push("/login");
            break;
          case NetworkError:
          default:
            setErr(["Network error is detected", Date.now()]);
            setLoading(false);
            break;
        }
      });
  }, [token])

  return (
    <>
    <Loading loading={loading} />
    <NoticeBar nonce={errNonce}>{errMsg}</NoticeBar>
    <Page>
      <Header />
      <Container>
        { me ? MeContents(me) : null }
      </Container>
    </Page>
    </>
  );
}

function MeContents(me) {
  const name = me.name;
  const email = me.email;
  const markets = me.markets;
  return (
    <>
    <Profile>
      <ProfileHeader>Profile</ProfileHeader>
      <ProfileItems>
        <ProfileItem>
          <ProfileItemKey>Name</ProfileItemKey>
          <ProfileItemVal>{name}</ProfileItemVal>
        </ProfileItem>
        <ProfileItem>
          <ProfileItemKey>Email</ProfileItemKey>
          <ProfileItemVal>{email}</ProfileItemVal>
        </ProfileItem>
      </ProfileItems>
    </Profile>
    <MarketList>
      <MarketListHeader>Markets</MarketListHeader>
      <MarketListItems>
      {markets.map(market =>
        <MarketListItem key={market.id}>
          <MarketListItemStatus>
            {market.status}
          </MarketListItemStatus>
          <MarketListItemTitle to={`/market/${market.id}`}>
            {market.title}
          </MarketListItemTitle>
        </MarketListItem>
      )}
      </MarketListItems>
    </MarketList>
    </>
  );
}


const Page = styled.div`
  width: 100vw;
  background-color: white;
`;

const Container = styled.div`
  width: 980px;
  margin: 0 auto;
  margin-top: 100px;
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
  align-items: flex-start;
`;

const Profile = styled.section`
  width: 300px;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
`;

const ProfileHeader = styled.h3`
  width: 100%;
  height: 40px;
  background-color: #F6F8FA;
  margin: 0;
  padding-left: 20px;
  font-size: 14px;
  color: #586069;
  line-height: 40px;
  border-bottom: 1px solid #D1D5DA;
  text-align: left;
`;

const ProfileItems = styled.ul`
  width: 100%;
  list-style: none;
  padding: 0;
  margin: 0;
`;

const ProfileItem = styled.li`
  width: 100%;
  padding: 10px 20px;
`;

const ProfileItemKey = styled.div`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 30%;
`

const ProfileItemVal = styled.div`
  display: inline-block;
  text-align: right;
  font-size: 14px;
  font-weight: bold;
  width: 70%;
`;

const MarketList = styled.section`
  width: 600px;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
`;

const MarketListHeader = styled.h3`
  width: 100%;
  height: 40px;
  font-size: 14px;
  color: #586069;
  background-color: #F6F8FA;
  line-height: 40px;
  padding-left: 20px;
  margin: 0px;
  border-bottom: 1px solid #D1D5DA;
`;

const MarketListItems = styled.ul`
  width: 100%;
  list-style: none;
  padding: 0;
  margin: 0;
`;

const MarketListItem = styled.li`
  padding: 10px;
  background-color: ${props => props.filled ? "#F9F9F9" : "white"};
`;

const MarketListItemStatus = styled.div`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 20%;
  padding-left: 20px;
`;

const MarketListItemTitle = styled(Link)`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 80%;
`;

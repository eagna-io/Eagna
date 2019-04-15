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
  const {accessToken, setAccessToken} = useContext(AccessTokenContext);
  const [me, setMe] = useState(null);
  const [loading, setLoading] = useState(false);
  const [[errMsg, errNonce], setErr] = useState([null, null]);

  useEffect(() => {
    if (!accessToken) {
      history.push("/login");
      return;
    }
    setLoading(true);
    getMe(accessToken)
      .then(me => {
        setMe(me);
        setLoading(false);
      })
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setAccessToken(null);
            history.push("/login");
            break;
          case NetworkError:
          default:
            setErr(["ネットワークエラー", Date.now()]);
            setLoading(false);
            break;
        }
      });
  }, [accessToken])

  return (
    <>
    <Loading loading={loading} />
    <NoticeBar nonce={errNonce}>{errMsg}</NoticeBar>
    <Page>
      <Header history={history} />
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
  const markets = me.markets.sort(sortMarket);
  return (
    <>
    <LeftContents>
      <Profile>
        <ProfileHeader>Profile</ProfileHeader>
        <ProfileItems>
          <tbody>
            <ProfileItem>
              <ProfileItemKey>Name</ProfileItemKey>
              <ProfileItemVal>{name}</ProfileItemVal>
            </ProfileItem>
            <ProfileItem>
              <ProfileItemKey>Email</ProfileItemKey>
              <ProfileItemVal>{email}</ProfileItemVal>
            </ProfileItem>
          </tbody>
        </ProfileItems>
      </Profile>
    </LeftContents>
    <RightContents>
      <MarketList>
        <MarketListHeader>Markets</MarketListHeader>
        <MarketListItems>
          <tbody>
          {markets.map(market =>
            <MarketListItem key={market.id}>
              <MarketListItemStatus>
                {market.status}
              </MarketListItemStatus>
              <td>
                <MarketListItemTitle to={`/market/${market.id}`}>
                {market.title}
                </MarketListItemTitle>
              </td>
            </MarketListItem>
          )}
          </tbody>
        </MarketListItems>
      </MarketList>
    </RightContents>
    </>
  );
}

function sortMarket(a, b) {
  const statusScore = status => {
    switch (status) {
      case "open": return 0;
      case "preparing": return 1;
      case "closed": return 2;
      case "settled": return 3;
    }
  }
  const statusDiff = statusScore(a.status) - statusScore(b.status);
  if (statusDiff === 0) {
    return a.id - b.id;
  } else {
    return statusDiff;
  }

}

const Page = styled.div`
  width: 100vw;
  background-color: white;
`;

const Container = styled.div`
  width: 90%;
  max-width: 980px;
  margin: 0 auto;
  margin-top: 100px;
  display: flex;
  flex-wrap: wrap;
  justify-content: space-around;
  align-items: flex-start;
`;

const LeftContents = styled.div`
  display: inline-block;
  width: 300px;
`;

const RightContents = styled.div`
  display: inline-block;
  width: 600px
`;

const Profile = styled.section`
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

const ProfileItems = styled.table`
  width: 100%;
  table-layout: fixed;
  border-spacing: 0;
  border-collapse: collapse;
  padding: 0;
  margin: 0;
`;

const ProfileItem = styled.tr`
  width: 100%;
`;

const ProfileItemKey = styled.td`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 30%;
  padding: 10px 0px;
  padding-left: 20px;
`

const ProfileItemVal = styled.td`
  display: inline-block;
  text-align: right;
  font-size: 14px;
  font-weight: bold;
  width: 70%;
  padding: 10px 0px;
  padding-right: 20px;
`;

const MarketList = styled.section`
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

const MarketListItems = styled.table`
  width: 100%;
  table-layout: fixed;
  border-spacing: 0;
  border-collapse: collapse;
  padding: 0;
  margin: 0;
`;

const MarketListItem = styled.tr`
  background-color: white;

  &:nth-child(even) {
    background-color: #F9F9F9;
  }
`;

const MarketListItemStatus = styled.td`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 20%;
  padding: 20px;
`;

const MarketListItemTitle = styled(Link)`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 80%;
`;

import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import Header from 'src/components/header';
import { requestMe } from '../actions';

class AccountPage extends React.Component {
  constructor(props) {
    super(props);
    if (this.props.accessToken) {
      this.props.requestMe(this.props.accessToken);
    }
  }

  render() {
    if (!this.props.accessToken) {
      return <Redirect to="/login" />
    }

    const name = this.props.name || "-";
    const email = this.props.email || ""
    const markets = this.props.markets || [];
    return (
      <Page>
        <Header />
        <Container>
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
                <MarketListItemTitle href={"/market/" + market.id}>
                  {market.title}
                </MarketListItemTitle>
              </MarketListItem>
            )}
            </MarketListItems>
          </MarketList>
        </Container>
      </Page>
    );
  }
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
  padding-left: 40px;
  font-size: 14px;
  font-weight: bold;
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
  font-weight: bold;
  background-color: #F6F8FA;
  line-height: 40px;
  padding-left: 40px;
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
  background-color: ${props => props.filled ? "#F9F9F9" : "white"}
`;

const MarketListItemStatus = styled.div`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 20%;
  padding-left: 20px;
`;

const MarketListItemTitle = styled.a`
  display: inline-block;
  text-align: left;
  font-size: 14px;
  width: 80%;
`;

function mapStateToProps(state) {
  return {
    ...state.pages.account,
    name: state.me.name,
    email: state.me.email,
    markets: state.me.markets,
    accessToken: state.me.accessToken,
  }
}

function mapDispatchToProps(dispatch) {
  return {
    requestMe: token => dispatch(requestMe(token))
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(AccountPage)

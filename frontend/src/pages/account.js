import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import { requestMe } from '../actions';

const Container = styled.div`
  width: 40vw;
  margin-left: 30vw;
  margin-top: 20vh;
`;

const Item = styled.div`
  height: 50px;
  border-bottom: solid 3px #84B6F9;
  margin-top: 60px;
  display: flex;
  justify-content: space-between;
  flex-direction: row;
  align-items: flex-end;
`;

const Key = styled.div`
  display: inline-block;
  text-align: left;
  color: #84B6F9;
  font-size: 20px;
  font-weight: bold;
`;

const Val = styled.div`
  display: inline-block;
  text-align: right;
  color: gray;
  font-size: 30px;
  font-weight: bold;
`;

const SmallText = styled.span`
  font-size: 20px;
  margin-left: 10px;
  font-weight: normal;
`;

const MarketList = styled.table`
  margin-top: 5px;
`;

const MarketListHeader = styled.th`
  width: 100%;
  background-color: #E0E0E0;
  color: gray;
`;

const MarketListItem = styled.td`
  padding: 5px;
  font-weight: bold;
`;

class AccountPage extends React.Component {
  constructor(props) {
    super(props);
    if (this.props.token != null) {
      this.props.requestMe(this.props.token);
    }
  }

  render() {
    if (this.props.token == null) {
      return <Redirect to="/login" />
    }
    const name = this.props.name == null ? "-" : this.props.name;
    const coins = this.props.coins == null ? 0 : this.props.coins;
    const markets = this.props.markets == null ? [] : this.props.markets;
    return (
      <Container>
        <Item>
          <Key>Name</Key>
          <Val>{name}</Val>
        </Item>
        <Item>
          <Key>Coins</Key>
          <Val>{coins}<SmallText>coins</SmallText></Val>
        </Item>
        <Item>
          <Key>Activities</Key>
        </Item>
        <MarketList>
          <thead>
            <tr>
              <MarketListHeader>Market</MarketListHeader>
              <MarketListHeader>Status</MarketListHeader>
            </tr>
          </thead>
          {markets.map(market =>
          <tbody key={market.title}>
            <tr>
              <MarketListItem>{market.title}</MarketListItem>
              <MarketListItem>{market.status}</MarketListItem>
            </tr>
          </tbody>
          )}
        </MarketList>
      </Container>
    );
  }
}

function mapStateToProps(state) {
  return {
    isRequesting: state.pages.me.isRequesting,
    token: state.me.accessToken,
    name: state.me.name,
    coins: state.me.coins,
    markets: state.me.markets,
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

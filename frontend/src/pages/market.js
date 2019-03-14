import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import Header from 'src/components/header';
import MarketHeader from './market/header';
import TokensComponent from './market/tokens';
import OrderComponent from './market/order';
import AssetsComponent from './market/assets';
import ResultComponent from './market/result';
import DescComponent from './market/description';

import { requestMarket, requestOrder } from '../actions';

/*
 market : {
   id         : int,
   title      : string,
   short_desc : string,
   desc       : string,
   open_ts    : int,
   close_ts   : int,
   status     : string,
   tokens     : [{
     id : int,
     name : string,
     desc : string,
     amount : int,
   }],
   me : {
     coins : int,
     tokens : [{
       token_id : int,
       amount : int,
     }],
   }
 }
 */
class MarketPage extends React.Component {
  constructor(props) {
    super(props)
    this.props.requestMarket(props.match.params.id, this.props.accessToken)
  }

  render() {
    if (this.props.needLogin) {
      return <Redirect to="/login" />
    }

    const market = this.props.market
    const marketHeaderDom = market == null ? null : (
      <MarketHeader
        title={market.title}
        shortDesc={market.short_desc}
        openTs={market.open_ts}
        closeTs={market.close_ts}
        status={market.status} />
    );
    const marketContentsDom = market == null ? null : (
      <Contents>
        <Tokens tokens={market.tokens} />
        {
          market.status === "open" ? (
            <OrderContainer>
              <OrderComponent
                tokens={market.tokens}
                accessToken={this.props.accessToken}
                marketId={this.props.market.id}
                requestOrder={this.props.requestOrder} />
              <AssetsComponent
                tokens={market.tokens}
                assets={market.me.tokens}
                coins={market.me.coins} />
            </OrderContainer>
          ) : market.status === "closed" ? (
            <OrderContainer>
              <ResultComponent result={market.result} />
              <AssetsComponent
                tokens={market.tokens}
                assets={market.me.tokens}
                coins={market.me.coins} />
            </OrderContainer>
          ) : null
        }
        <Description content={market.desc}/>
      </Contents>
    );

    return (
      <Page>
        <Header />
        { marketHeaderDom }
        { marketContentsDom }
      </Page>
    )
  }
}


const Page = styled.div`
  width: 100vw;
  background-color: white;
`;

const Contents = styled.div`
  width: 980px;
  margin: 0 auto;
  padding-bottom: 50px;
`;

const Tokens = styled(TokensComponent)`
  margin-top: 50px;
`;

const OrderContainer = styled.div`
  display: flex;
  justify-content: space-between;
  margin-top: 50px;
`;

const Description = styled(DescComponent)`
  margin-top: 50px;
`;


function mapStateToProps(state) {
  return {
    ...state.pages.market,
    accessToken: state.me.accessToken,
  }
}

function mapDispatchToProps(dispatch) {
  return {
    requestMarket: (marketId, accessToken) =>
      dispatch(requestMarket(marketId, accessToken)),
    requestOrder: (marketId, tokenId, amountToken, amountCoin, accessToken) =>
      dispatch(requestOrder(marketId, tokenId, amountToken, amountCoin, accessToken)),
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(MarketPage)

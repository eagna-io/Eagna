import React, {FC} from 'react';
import styled from 'styled-components';
import moment from 'moment';

import {
  Market,
  UpcomingMarket,
  OpenMarket,
  ClosedMarket,
  ResolvedMarket,
  Token,
} from 'models/market';
import {User} from 'models/user';
import {MyAssets, NormalOrder} from 'models/order';
import {createNormalOrder} from 'api/market';
import {LoginStatus, withUser} from 'app/components/user';
import {pc} from 'app/components/responsive';

interface OrderComponentProps {
  token: Token;
  market: Market;
  myAssets: MyAssets | null;
  user: LoginStatus;
}

const OrderComponent: FC<OrderComponentProps> = ({
  token,
  market,
  myAssets,
  user,
}) => {
  let msg: string | null = null;
  let buyPrice: number | null = null;
  let buyable = false;
  let sellPrice: number | null = null;
  let sellable = false;
  let requestBuy: () => void = () => {};
  let requestSell: () => void = () => {};

  if (market instanceof UpcomingMarket) {
    msg = 'マーケットがOpen状態になると、取引が行えるようになります';
  } else if (market instanceof OpenMarket) {
    const buyAmountCoin = market.computeAmountCoinOfOrder(token.name, 1);
    const sellAmountCoin = market.computeAmountCoinOfOrder(token.name, -1);
    buyPrice = -buyAmountCoin;
    sellPrice = sellAmountCoin;

    if (myAssets === null) {
      msg = '「参加する」ボタンを押すと取引ができるようになります';
    } else {
      buyable = myAssets.getCoin() >= buyPrice;
      sellable = myAssets.getTokenUncheck(token.name) >= 1;
    }

    if (user instanceof User) {
      const requestOrder = (amountToken: number, amountCoin: number) => {
        user.getAccessToken().then(accessToken => {
          if (accessToken === null) {
            alert('もう一度ログインをお願いいたします。');
          } else {
            createNormalOrder(
              market.id,
              accessToken,
              new NormalOrder(token.name, amountToken, amountCoin, moment()),
            ).then(res => {
              console.log(res);
              window.location.reload();
            });
          }
        });
      };
      requestBuy = () => requestOrder(1, buyAmountCoin);
      requestSell = () => requestOrder(-1, sellAmountCoin);
    }
  } else if (
    market instanceof ClosedMarket ||
    market instanceof ResolvedMarket
  ) {
    msg = 'マーケットはすでにCloseしています';
  }

  return (
    <>
      {msg !== null ? <OrderNote>{msg}</OrderNote> : null}
      <OrderContainer>
        <OrderButtonContainer>
          <BuyButton disabled={!buyable} onClick={requestBuy}>
            {buyPrice === null ? '-' : buyPrice}
            <OrderButtonTextUnit>coin</OrderButtonTextUnit>
          </BuyButton>
          <OrderButtonDesc>で購入する</OrderButtonDesc>
        </OrderButtonContainer>
        <OrderButtonContainer>
          <SellButton disabled={!sellable} onClick={requestSell}>
            {sellPrice === null ? '-' : sellPrice}
            <OrderButtonTextUnit>coin</OrderButtonTextUnit>
          </SellButton>
          <OrderButtonDesc>で売却する</OrderButtonDesc>
        </OrderButtonContainer>
      </OrderContainer>
    </>
  );
};

export default withUser(OrderComponent);

const OrderNote = styled.p`
  width: 100%;
  font-size: 12px;
  color: #ff6666;
  padding: 0;
  margin: 0;
  margin-top: 30px;
  text-align: center;

  ${pc(`
    margin-top: 50px;
  `)};
`;

const OrderContainer = styled.div`
  display: flex;
  width: 100%;
  justify-content: space-around;
  margin-top: 15px;
  padding: 0px 2% 25px 2%;
`;

const OrderButtonContainer = styled.div`
  width: 130px;
`;

const OrderButton = styled.button`
  display: block;
  width: 100%;
  height: 50px;
  box-shadow: 1px 1px 4px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;
  border: none;
  font-size: 18px;
  font-weight: bold;
  color: white;
  text-align: center;

  &:disabled {
    background-color: #9b9b9b;
  }
`;

const BuyButton = styled(OrderButton)`
  background-color: #358ed7;
`;

const SellButton = styled(OrderButton)`
  background-color: #d75035;
`;

const OrderButtonTextUnit = styled.span`
  font-size: 14px;
  font-weight: normal;
  margin-left: 5px;
`;

const OrderButtonDesc = styled.div`
  width: 100%;
  font-size: 12px;
  color: #979797;
  text-align: right;
  margin-top: 5px;
`;

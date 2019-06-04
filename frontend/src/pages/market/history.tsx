import React, {FC} from 'react';
import styled from 'styled-components';

import {Token, Order} from 'models/market';

interface TradeHistoryComponentProps {
  tokens: Token[];
  myOrders: Order[];
  className?: string;
}

const TradeHistoryComponent: FC<TradeHistoryComponentProps> = ({
  tokens,
  myOrders,
  className,
}) => {
  return (
    <Table className={className}>
      <thead>
        <Header>
          <HeaderCol>Time</HeaderCol>
          <HeaderCol>Types</HeaderCol>
          <HeaderCol>Token</HeaderCol>
          <HeaderCol right>Amount Token</HeaderCol>
          <HeaderCol right>Amount Coin</HeaderCol>
        </Header>
      </thead>
      <tbody>
        {myOrders
          .sort((a, b) => b.time.unix() - a.time.unix())
          .map(order => {
            let orderType: string = order.type;
            if (orderType === 'Normal') {
              orderType = order.amountToken < 0 ? 'Sell' : 'Buy';
            }

            let tokenName = '-';
            if (order.type === 'Normal' || order.type === 'Settle') {
              const token = tokens.find(t => t.id === order.tokenId);
              if (token === undefined) {
                throw `Order to non-exist token : ${order.tokenId}`;
              } else {
                tokenName = token.name;
              }
            }

            return (
              <Item bold={order.type !== 'Normal'} key={order.time.unix()}>
                <ItemCol>{order.time.toISOString()}</ItemCol>
                <ItemCol>{orderType}</ItemCol>
                <ItemCol>{tokenName}</ItemCol>
                <ItemCol right>{order.amountToken}</ItemCol>
                <ItemCol right>{order.amountCoin}</ItemCol>
              </Item>
            );
          })}
      </tbody>
    </Table>
  );
};

export default TradeHistoryComponent;

const Table = styled.table`
  table-layout: fixed;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  border-spacing: 0;
  border-collapse: collapse;
`;

const Header = styled.tr`
  background-color: #f6f8fa;
  height: 40px;
  border: none;
`;

const HeaderCol = styled('th')<{right?: boolean}>`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  text-align: ${props => (props.right ? 'right' : 'left')};
  padding: 0px 30px;
`;

const Item = styled('tr')<{bold?: boolean}>`
  height: 50px;
  border-top: 1px solid #d1d5da;
  background-color: white;
  font-weight: ${props => (props.bold ? 'bold' : 'normal')};

  &:nth-child(even) {
    background-color: #f9f9f9;
  }
`;

const ItemCol = styled('td')<{right?: boolean}>`
  font-size: 12px;
  font-family: Lucida Grande;
  text-align: ${props => (props.right ? 'right' : 'left')};
  padding: 0px 30px;
`;

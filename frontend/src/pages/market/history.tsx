import React, {FC} from 'react';
import styled from 'styled-components';

import * as table from 'components/table';
import {Token, MyOrderHistory} from 'models/market';

interface TradeHistoryComponentProps {
  tokens: Token[];
  myOrders: MyOrderHistory;
  maxHeight?: number;
  className?: string;
}

const TradeHistoryComponent: FC<TradeHistoryComponentProps> = ({
  tokens,
  myOrders,
  maxHeight,
  className,
}) => {
  return (
    <table.Table className={className} maxHeight={maxHeight}>
      <table.Header>
        <table.Cell2 bold small>
          Time
        </table.Cell2>
        <table.Cell2 bold small>
          Types
        </table.Cell2>
        <table.Cell2 bold small>
          Token
        </table.Cell2>
        <table.Cell2 bold small right>
          Amount Token
        </table.Cell2>
        <table.Cell2 bold small right>
          Amount Coin
        </table.Cell2>
      </table.Header>
      <table.Body>
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
              <table.Row striped key={order.time.unix()}>
                <table.Cell2>{order.time.fromNow()}</table.Cell2>
                <table.Cell2>{orderType}</table.Cell2>
                <table.Cell2>{tokenName}</table.Cell2>
                <table.Cell2 right>{order.amountToken}</table.Cell2>
                <table.Cell2 right>{order.amountCoin}</table.Cell2>
              </table.Row>
            );
          })}
      </table.Body>
    </table.Table>
  );
};

export default TradeHistoryComponent;

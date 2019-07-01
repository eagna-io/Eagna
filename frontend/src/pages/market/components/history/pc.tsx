import React, {FC} from 'react';
import styled from 'styled-components';

import * as table from 'components/table';
import {Token, MyOrderHistory, orderId} from 'models/market';
import {orderTypeStr} from '../history';

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
    <Container>
      <table.Table className={className} striped>
        <table.Header>
          <table.Cell2>時間</table.Cell2>
          <table.Cell2>種類</table.Cell2>
          <table.Cell2>トークン</table.Cell2>
          <table.Cell2>
            <Amount>トークン数</Amount>
          </table.Cell2>
          <table.Cell2>
            <Amount>コイン量</Amount>
          </table.Cell2>
        </table.Header>
        <table.Body>
          {myOrders
            .sort((a, b) => b.time.unix() - a.time.unix())
            .map(order => {
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
                <table.Row key={orderId(order)}>
                  <table.Cell2>{order.time.fromNow()}</table.Cell2>
                  <table.Cell2>{orderTypeStr(order)}</table.Cell2>
                  <table.Cell2>{tokenName}</table.Cell2>
                  <table.Cell2>
                    <Amount>{order.amountToken}</Amount>
                  </table.Cell2>
                  <table.Cell2>
                    <Amount>{order.amountCoin}</Amount>
                  </table.Cell2>
                </table.Row>
              );
            })}
        </table.Body>
      </table.Table>
    </Container>
  );
};

const Container = styled.div`
  width: 100%;
  margin-top: 50px;
`;

const Amount = styled.div`
  text-align: right;
`;

export default TradeHistoryComponent;

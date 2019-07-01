import React, {FC} from 'react';
import styled from 'styled-components';

import * as table from 'components/table';
import {Token, MyOrderHistory, orderId, Order} from 'models/market';

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
          <table.Cell2>
            <Item>時間</Item>
          </table.Cell2>
          <table.Cell2>
            <Item>種類</Item>
          </table.Cell2>
          <table.Cell2>
            <Item>トークン</Item>
          </table.Cell2>
          <table.Cell2>
            <Item>トークン数</Item>
          </table.Cell2>
          <table.Cell2>
            <Item>コイン量</Item>
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
                  <table.Cell2>
                    <Item>{order.time.fromNow()}</Item>
                  </table.Cell2>
                  <table.Cell2>
                    <Item>{orderTypeStr(order)}</Item>
                  </table.Cell2>
                  <table.Cell2>
                    <Item>{tokenName}</Item>
                  </table.Cell2>
                  <table.Cell2>
                    <Item>{order.amountToken}</Item>
                  </table.Cell2>
                  <table.Cell2>
                    <Item>{order.amountCoin}</Item>
                  </table.Cell2>
                </table.Row>
              );
            })}
        </table.Body>
      </table.Table>
    </Container>
  );
};

function orderTypeStr(order: Order): string {
  if (order.type === 'Normal') {
    return order.amountToken < 0 ? '売り' : '買い';
  } else if (order.type === 'InitialSupply') {
    return '初期配布';
  } else  {
    return order.amountCoin === 0 ? '没収' : '報酬';
  }
}

const Container = styled.div`
  width: 100%;
  margin-top: 50px;
`;

const Amount = styled.div`
  text-align: right;
`;

const Item = styled.div`
  font-size: 11px;
`;

export default TradeHistoryComponent;

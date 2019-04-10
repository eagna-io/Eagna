import React from 'react';
import styled from 'styled-components';

import { dateToStr } from 'src/time';


export default function TradeHistory(props) {
  const tokens = props.tokens;
  const orderHistory = props.orderHistory;

  return (
    <Table className={props.className}>
      <thead>
        <Header>
          <HeaderCol>Time</HeaderCol>
          <HeaderCol>Types</HeaderCol>
          <HeaderCol>Token</HeaderCol>
          <HeaderCol>Amount Token</HeaderCol>
          <HeaderCol>Amount Coin</HeaderCol>
        </Header>
      </thead>
      <tbody>
        {
          orderHistory
            .records
            .sort((a,b) => b.time - a.time)
            .map(order => {
              const orderType =
                order.type === "normal"
                  ? order.amountToken < 0 ? "sell" : "buy"
                  : order.type
              const tokenName = order.token ? order.token.name : "-"
              return (
                <Item bold={order.type !== "normal"} key={order.id}>
                  <ItemCol>{dateToStr(order.time)}</ItemCol>
                  <ItemCol>{orderType}</ItemCol>
                  <ItemCol>{tokenName}</ItemCol>
                  <ItemCol>{order.amountToken}</ItemCol>
                  <ItemCol>{order.amountCoin}</ItemCol>
                </Item>
              )
            })
        }
      </tbody>
    </Table>
  );
}

const Table = styled.table`
  table-layout: fixed;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
  border-spacing: 0;
  border-collapse: collapse;
`;

const Header = styled.tr`
  background-color: #F6F8FA;
  height: 40px;
  border: none;
`;

const HeaderCol = styled.th`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  text-align: left;
  padding-left: 30px;
`;

const Item = styled.tr`
  height: 50px;
  border-top: 1px solid #D1D5DA;
  background-color: white;
  font-weight: ${props => props.bold ? "bold" : "normal" };

  &:nth-child(even) {
    background-color: #F9F9F9;
  }
`;

const ItemCol = styled.td`
  font-size: 12px;
  font-family: Lucida Grande;
  padding-left: 30px;
`;

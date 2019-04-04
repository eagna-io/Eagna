import React from 'react';
import styled from 'styled-components';
import * as lmsr from 'src/lmsr';

export default function Tokens(props) {
  const prices = lmsr.prices(props.tokens.map(t => t.amount))
  const tokens = props.tokens;

  return (
    <Table className={props.className}>
      <thead>
        <Header>
          <HeaderToken>Token</HeaderToken>
          <HeaderPrice>Price</HeaderPrice>
          <HeaderDesc>Description</HeaderDesc>
        </Header>
      </thead>
      <tbody>
        {
          tokens.map((token, idx) => (
          <Item key={token.name}>
            <ItemToken>{token.name}</ItemToken>
            <ItemPrice>{prices[idx]}</ItemPrice>
            <ItemDesc>{token.desc}</ItemDesc>
          </Item>
          ))
        }
      </tbody>
    </Table>
  );
}

const Table = styled.table`
  table-layout: fixed;
  width: 100%;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
  border-spacing: 0;
  border-collapse: collapse;
`;

const Header = styled.tr`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  font-weight: normal;
  background-color: #F6F8FA;
  height: 40px;
  border: none;
`;

const HeaderToken = styled.th`
  padding-left: 75px;
  text-align: left;
  width: 180px;
`;

const HeaderPrice = styled.th`
  width: 100px;
  text-align: right;
`;

const HeaderDesc = styled.th`
  padding-left: 100px;
  padding-right: 30px;
  text-align: left;
`;

const Item = styled.tr`
  height: 50px;
  border-top: 1px solid #D1D5DA;
  background-color: white;

  &:nth-child(even) {
    background-color: #F9F9F9;
  }
`;

const ItemToken = styled.td`
  color: #37474F;
  font-size: 14px;
  font-family: Lucida Grande;
  font-weight: normal;
  padding-left: 75px;
`;

const ItemPrice = styled.td`
  color: #37474F;
  font-size: 16px;
  font-family: Lucida Grande;
  font-weight: normal;
  text-align: right;
`;

const ItemDesc = styled.td`
  padding-left: 100px;
  padding-right: 30px;
  color: #979797;
  font-size: 14px;
  font-family: Lucida Grande;
  font-weight: normal;
  text-align: left;
`;

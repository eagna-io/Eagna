import React, {FC} from 'react';
import styled from 'styled-components';

import {Token, TokenPrices} from 'models/market';

interface TokensComponentProps {
  tokens: Token[];
  tokenPrices: TokenPrices | null;
  className?: string;
}

const TokensComponent: FC<TokensComponentProps> = ({
  tokens,
  tokenPrices,
  className,
}) => {
  return (
    <Table className={className}>
      <thead>
        <Header>
          <HeaderToken>Token</HeaderToken>
          <HeaderPrice>Price</HeaderPrice>
          <HeaderDesc>Description</HeaderDesc>
        </Header>
      </thead>
      <tbody>
        {tokens.map(token => (
          <Item key={token.name}>
            <ItemToken>{token.name}</ItemToken>
            <ItemPrice>{tokenPrices ? tokenPrices.get(token.id) : "-"}</ItemPrice>
            <ItemDesc>{token.description}</ItemDesc>
          </Item>
        ))}
      </tbody>
    </Table>
  );
};

export default TokensComponent;

const Table = styled.table`
  table-layout: fixed;
  width: 100%;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  border-spacing: 0;
  border-collapse: collapse;
`;

const Header = styled.tr`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  font-weight: normal;
  background-color: #f6f8fa;
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
  border-top: 1px solid #d1d5da;
  background-color: white;

  &:nth-child(even) {
    background-color: #f9f9f9;
  }
`;

const ItemToken = styled.td`
  color: #37474f;
  font-size: 14px;
  font-family: Lucida Grande;
  font-weight: normal;
  padding-left: 75px;
`;

const ItemPrice = styled.td`
  color: #37474f;
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

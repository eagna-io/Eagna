import React from 'react';
import styled from 'styled-components';

export default function Assets(props) {
  const tokens = props.tokens;
  const orderHistory = props.orderHistory;

  return (
    <Table className={props.className}>
      <thead>
        <Header>
          <HeaderAsset>Asset</HeaderAsset>
          <HeaderVolume>Volume</HeaderVolume>
        </Header>
      </thead>
      <tbody>
        <AssetItem coin key={"coin"}>
          <AssetLabel>{"Coin"}</AssetLabel>
          <ItemVolume>{orderHistory.currentAmountCoin()}</ItemVolume>
        </AssetItem>
        {
          tokens.map(token => {
            return (
              <AssetItem key={token.id}>
                <AssetLabel>{token.name}</AssetLabel>
                <ItemVolume>{orderHistory.currentAmountToken(token)}</ItemVolume>
              </AssetItem>
            )
          })
        }
      </tbody>
    </Table>
  );
}

const Table = styled.table`
  table-layout: fixed;
  width: 406px;
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

const HeaderAsset = styled.th`
  text-align: left;
  padding-left: 75px;
`;

const HeaderVolume = styled.th`
  text-align: right;
  padding-right: 75px;
`;

const AssetItem = styled.tr`
  height: 50px;
  border-top: 1px solid #D1D5DA;
  background-color: white;
  color: #37474F;
  font-size: ${props => props.coin ? "16px" : "14px"};
  font-family: Lucida Grande;
  font-weight: ${props => props.coin ? "bold" : "normal"};

  &:nth-child(even) {
    background-color: #F9F9F9;
  }
`;

const AssetLabel = styled.td`
  text-align: left;
  padding-left: 75px;
`;

const ItemVolume = styled.td`
  text-align: right;
  padding-right: 75px;
`;

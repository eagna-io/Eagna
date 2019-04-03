import React from 'react';
import styled from 'styled-components';

export default function Assets(props) {
  const tokens = props.tokens;
  const orders = props.orders;

  const coins = orders.reduce((acc, order) => acc + order.amountCoin, 0);

  return (
    <Container className={props.className}>
      <thead>
        <Header>
          <HeaderAsset>Asset</HeaderAsset>
          <HeaderVolume>Volume</HeaderVolume>
        </Header>
      </thead>
      <tbody>
        <AssetItem key={"coin"}>
          <AssetLabel coin={true}>{"Coin"}</AssetLabel>
          <ItemVolume>{coins}</ItemVolume>
        </AssetItem>
        {
          tokens.map(token => {
            const amountToken =
              orders
                .filter(o => o.tokenId === token.id)
                .reduce((acc, order) => acc + order.amountToken, 0);
            return (
              <AssetItem key={token.id}>
                <AssetLabel coin={false}>{token.name}</AssetLabel>
                <ItemVolume>{amountToken}</ItemVolume>
              </AssetItem>
            )
          })
        }
      </tbody>
    </Container>
  );
}

const Container = styled.table`
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
  width: 70%;
  text-align: left;
  padding-left: 75px;
`;

const HeaderVolume = styled.th`
  width: 30%;
  text-align: right;
  padding-right: 75px;
`;

const AssetItem = styled.tr`
  height: 50px;
  border-top: 1px solid #D1D5DA;
  background-color: white;

  &:nth-child(even) {
    background-color: #F9F9F9;
  }
`;

const AssetLabel = styled.td`
  color: #37474F;
  font-size: ${props => props.coin ? "16px" : "14px"};
  font-family: Lucida Grande;
  font-weight: ${props => props.coin ? "bold" : "normal"};
  padding-left: 75px;
`;

const ItemVolume = styled.td`
  color: #37474F;
  font-size: 16px;
  font-family: Lucida Grande;
  font-weight: normal;
  text-align: right;
  padding-right: 75px;
`;

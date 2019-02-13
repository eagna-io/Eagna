import React from 'react';
import styled from 'styled-components';

export default function Assets(props) {
  return (
    <Container className={props.className}>
      <thead>
        <Header>
          <HeaderAsset>Asset</HeaderAsset>
          <HeaderVolume>Volume</HeaderVolume>
        </Header>
      </thead>
      <tbody>
        {
          props.assets.map((asset, idx) => (
          <Item filled={idx % 2 == 1} key={asset.name}>
            <ItemAsset>{asset.name}</ItemAsset>
            <ItemVolume>{asset.volume}</ItemVolume>
          </Item>
          ))
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

const Item = styled.tr`
  height: 50px;
  border-top: 1px solid #D1D5DA;
  background-color: ${props => props.filled ? "#F9F9F9" : "white" };
`;

const ItemAsset = styled.td`
  color: #37474F;
  font-size: 14px;
  font-family: Lucida Grande;
  font-weight: normal;
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

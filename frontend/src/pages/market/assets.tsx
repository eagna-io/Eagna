import React, {FC} from 'react';
import styled from 'styled-components';

import {
  Token,
  MyAssets,
} from 'models/market';

interface AssetsComponentProps {
  tokens: Token[];
  myAssets: MyAssets;
  className?: string;
}

const AssetsComponent: FC<AssetsComponentProps> = ({
  tokens,
  myAssets,
  className,
}) => {
  return (
    <Table className={className}>
      <thead>
        <Header>
          <HeaderAsset>Asset</HeaderAsset>
          <HeaderVolume>Volume</HeaderVolume>
        </Header>
      </thead>
      <tbody>
        <AssetItem coin key={'coin'}>
          <AssetLabel>{'Coin'}</AssetLabel>
          <ItemVolume>{myAssets.get('Coin')}</ItemVolume>
        </AssetItem>
        {tokens.map(token => {
          return (
            <AssetItem key={token.id}>
              <AssetLabel>{token.name}</AssetLabel>
              <ItemVolume>{myAssets.get(token.id)}</ItemVolume>
            </AssetItem>
          );
        })}
      </tbody>
    </Table>
  );
};

export default AssetsComponent;

const Table = styled.table`
  table-layout: fixed;
  width: 406px;
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

const HeaderAsset = styled.th`
  text-align: left;
  padding-left: 75px;
`;

const HeaderVolume = styled.th`
  text-align: right;
  padding-right: 75px;
`;

const AssetItem = styled('tr')<{coin?: boolean}>`
  height: 50px;
  border-top: 1px solid #d1d5da;
  background-color: white;
  color: #37474f;
  font-size: ${props => (props.coin ? '16px' : '14px')};
  font-family: Lucida Grande;
  font-weight: ${props => (props.coin ? 'bold' : 'normal')};

  &:nth-child(even) {
    background-color: #f9f9f9;
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

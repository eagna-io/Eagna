import React, {FC} from 'react';
import styled from 'styled-components';

import * as table from 'components/table';
import {Token, MyAssets} from 'models/market';

interface AssetsComponentProps {
  tokens: Token[];
  myAssets: MyAssets;
  maxHeight?: number;
  className?: string;
}

export const Pc: FC<AssetsComponentProps> = ({
  tokens,
  myAssets,
  maxHeight,
  className,
}) => {
  const Table = styled(table.Table)`
    width: 406px;
    margin-top: 50px;
  `;
  return (
    <Table className={className} striped>
      <table.Header>
        <table.Cell6>コイン / トークン</table.Cell6>
        <table.Cell4>量</table.Cell4>
      </table.Header>
      <table.Body>
        <table.Row>
          <table.Cell6>{'Coin'}</table.Cell6>
          <table.Cell4>{myAssets.get('Coin')}</table.Cell4>
        </table.Row>
        {tokens.map(token => {
          return (
            <table.Row key={token.id}>
              <table.Cell6>{token.name}</table.Cell6>
              <table.Cell4>{myAssets.get(token.id)}</table.Cell4>
            </table.Row>
          );
        })}
      </table.Body>
    </Table>
  );
};

export const Mobile: FC<AssetsComponentProps> = ({
  tokens,
  myAssets,
  maxHeight,
  className,
}) => {
  const Table = styled(table.Table)`
    width: 100%;
    margin-top: 50px;
  `;
  return (
    <Table className={className} striped>
      <table.Header>
        <table.Cell6>コイン / トークン</table.Cell6>
        <table.Cell4>量</table.Cell4>
      </table.Header>
      <table.Body>
        <table.Row>
          <table.Cell6>{'Coin'}</table.Cell6>
          <table.Cell4>{myAssets.get('Coin')}</table.Cell4>
        </table.Row>
        {tokens.map(token => {
          return (
            <table.Row key={token.id}>
              <table.Cell6>{token.name}</table.Cell6>
              <table.Cell4>{myAssets.get(token.id)}</table.Cell4>
            </table.Row>
          );
        })}
      </table.Body>
    </Table>
  );
};

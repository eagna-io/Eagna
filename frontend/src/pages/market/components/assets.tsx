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

const AssetsComponent: FC<AssetsComponentProps> = ({
  tokens,
  myAssets,
  maxHeight,
  className,
}) => {
  return (
    <Table className={className} maxHeight={maxHeight}>
      <table.Header>
        <table.Cell6 bold small>
          Asset
        </table.Cell6>
        <table.Cell4 bold small right>
          Volume
        </table.Cell4>
      </table.Header>
      <table.Body>
        <table.Row striped>
          <table.Cell6 bold>{'Coin'}</table.Cell6>
          <table.Cell4 bold right>
            {myAssets.get('Coin')}
          </table.Cell4>
        </table.Row>
        {tokens.map(token => {
          return (
            <table.Row striped key={token.id}>
              <table.Cell6>{token.name}</table.Cell6>
              <table.Cell4 right>{myAssets.get(token.id)}</table.Cell4>
            </table.Row>
          );
        })}
      </table.Body>
    </Table>
  );
};

export default AssetsComponent;

const Table = styled(table.Table)`
  width: 406px;
`;

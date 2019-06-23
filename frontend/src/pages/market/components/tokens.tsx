import React, {FC} from 'react';
import styled from 'styled-components';

import {Token, TokenPrices} from 'models/market';
import * as table from 'components/table';

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
    <table.Table className={className}>
      <table.Header>
        <table.Cell3 bold small>
          Token
        </table.Cell3>
        <table.Cell1 bold small right>
          Price
        </table.Cell1>
        <table.Cell6 bold small>
          Description
        </table.Cell6>
      </table.Header>
      <table.Body>
        {tokens.map(token => (
          <table.Row key={token.name} striped>
            <table.Cell3>
              <TokenName>{token.name}</TokenName>
            </table.Cell3>
            <table.Cell1 right>
              <TokenPrice>
                {tokenPrices ? tokenPrices.get(token.id) : '-'}
              </TokenPrice>
            </table.Cell1>
            <table.Cell6>
              <TokenDesc>{token.description}</TokenDesc>
            </table.Cell6>
          </table.Row>
        ))}
      </table.Body>
    </table.Table>
  );
};

export default TokensComponent;

const TokenName = styled.div`
  color: #37474f;
`;

const TokenPrice = styled.div`
  color: #37474f;
  font-size: 16px;
`;

const TokenDesc = styled.div`
  color: #979797;
`;

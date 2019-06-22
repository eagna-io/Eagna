import React, {FC} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';

import {Market} from 'models/market';
import * as table from 'components/table';

export const Pc: FC<{markets: Market[]; title: string}> = ({
  markets,
  title,
}) => {
  const MarketStatus = styled.div`
    font-size: 14px;
  `;

  const MarketTitle = styled(Link)`
    font-size: 16px;
    font-weight: bold;
  `;

  return (
    <table.Table>
      <table.Header>
        <table.Cell>{title}</table.Cell>
      </table.Header>
      <table.Body>
        {markets.map(market => (
          <table.Row key={market.id} striped={true}>
            <table.Cell3>
              <MarketStatus>{market.status}</MarketStatus>
            </table.Cell3>
            <table.Cell7>
              <MarketTitle to={`/market/${market.id}`}>
                {market.title}
              </MarketTitle>
            </table.Cell7>
          </table.Row>
        ))}
      </table.Body>
    </table.Table>
  );
};

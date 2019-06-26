import React, {FC} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';

import {Market} from 'models/market';
import StatusBadge from 'components/status_badge';
import * as table from 'components/table';

const Markets: FC<{markets: Market[]; title: string}> = React.memo(
  ({markets, title}) => {
    return (
      <table.Table striped>
        <table.Header>
          <table.Cell>
            <TableTitle>{title}</TableTitle>
          </table.Cell>
        </table.Header>
        <table.Body>
          {markets.map(market => (
            <table.Row key={market.id}>
              <table.Cell3>
                <StyledStatusBadge status={market.status} />
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
  },
);

const TableTitle = styled.h3`
  margin: 0;
  padding: 0;
  font-size: 13px;
  font-weight: normal;
`;

const StyledStatusBadge = styled(StatusBadge)`
  font-size: 10px;
  height: 20px;
  line-height: 20px;
`;

const MarketTitle = styled(Link)`
  font-size: 12px;
  font-weight: bold;
`;

export default Markets;

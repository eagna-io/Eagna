import React, {FC} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';

import {Market} from 'models/market';
import StatusBadge from 'components/status_badge';
import * as table from 'components/table';

const Markets: FC<{markets: Market[]; title: string}> = React.memo(
  ({markets, title}) => {
    return (
      <StyledTable striped>
        <table.Header>
          <table.Cell>{title}</table.Cell>
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
      </StyledTable>
    );
  },
);

const StyledTable = styled(table.Table)`
  max-height: 300px;
`;

const StyledStatusBadge = styled(StatusBadge)`
  height: 25px;
  line-height: 25px;
  font-size: 14px;
`;

const MarketTitle = styled(Link)`
  font-size: 16px;
`;

export default Markets;

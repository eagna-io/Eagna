import React, {FC} from 'react';
import {History} from 'history';

import {MarketId} from 'models/market';
import User from 'models/user';
import {Pc, Tablet, Mobile} from 'components/responsive';
import PcMarketPage from './market/pc';
import MobileMarketPage from './market/mobile';

interface MarketPageProps {
  history: History;
  user: User | null;
  marketId: MarketId;
}

const MarketPage: FC<MarketPageProps> = ({history, user, marketId}) => {
  return (
    <>
      <Pc>
        <PcMarketPage history={history} user={user} marketId={marketId} />
      </Pc>
      <Tablet>
        <MobileMarketPage history={history} user={user} marketId={marketId} />
      </Tablet>
      <Mobile>
        <MobileMarketPage history={history} user={user} marketId={marketId} />
      </Mobile>
    </>
  );
};

export default MarketPage;

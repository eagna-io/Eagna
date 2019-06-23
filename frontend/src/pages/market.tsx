import React, {FC} from 'react';
import {History} from 'history';

import {MarketId} from 'models/market';
import User from 'models/user';
import Responsive from 'components/responsive';
import PcMarketPage from './market/pc';
import MobileMarketPage from './market/mobile';

interface MarketPageProps {
  history: History;
  user: User | null;
  marketId: MarketId;
}

const MarketPage: FC<MarketPageProps> = ({history, user, marketId}) => {
  return (
    <Responsive
      renderPc={() => (
        <PcMarketPage history={history} user={user} marketId={marketId} />
      )}
      renderTablet={() => (
        <MobileMarketPage history={history} user={user} marketId={marketId} />
      )}
      renderMobile={() => (
        <MobileMarketPage history={history} user={user} marketId={marketId} />
      )}
    />
  );
};

export default MarketPage;

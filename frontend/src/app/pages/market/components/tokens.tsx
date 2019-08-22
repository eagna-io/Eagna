import React, {FC} from 'react';

import {Market} from 'models/market';
import {PriceHistory, MyAssets} from 'models/order';
import {Pc, Mobile, Tablet} from 'app/components/responsive';

import MobileComponent from './tokens/mobile';
import PcComponent from './tokens/pc';

export interface TokenListComponentProps {
  market: Market;
  priceHistory: PriceHistory | null;
  myAssets: MyAssets | null;
}

const TokenListComponent: FC<TokenListComponentProps> = props => {
  return (
    <>
      <Mobile>
        <MobileComponent {...props} />
      </Mobile>
      <Tablet>
        <MobileComponent {...props} />
      </Tablet>
      <Pc>
        <PcComponent {...props} />
      </Pc>
    </>
  );
};

export default TokenListComponent;
import PcHistory from './history/pc';
import MobileHistory from './history/mobile';

import {Order} from 'models/market';

export const Pc = PcHistory;
export const Mobile = MobileHistory;

export function orderTypeStr(order: Order): string {
  if (order.type === 'Normal') {
    return order.amountToken < 0 ? '売り' : '買い';
  } else if (order.type === 'InitialSupply') {
    return '初期配布';
  } else {
    return order.amountCoin === 0 ? '没収' : '報酬';
  }
}

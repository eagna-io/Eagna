import React, {FC} from 'react';
import styled from 'styled-components';

import {pc} from 'components/responsive';
import {MarketStatus} from 'models/market';

interface StatusBadgeProps {
  status: MarketStatus;
  className?: string;
}

const StatusBadge: FC<StatusBadgeProps> = ({status, className}) => {
  const color = statusToColor(status);

  const Badge = styled.div`
    width: 72p;
    height: 27px;
    line-height: 27px;
    font-size: 11px;
    background-color: ${color};
    border-radius: 4px;
    color: white;
    font-weight: 700;
    text-align: center;

    ${pc(`
    width: 87px;
    font-size: 15px;
  `)}
  `;

  return <Badge className={className}>{status}</Badge>;
};

export default StatusBadge;

function statusToColor(s: MarketStatus): string {
  if (s === MarketStatus.Upcoming) {
    return '#D8D212';
  } else if (s === MarketStatus.Open) {
    return '#23AC0E';
  } else if (s === MarketStatus.Closed) {
    return '#3261AB';
  } else {
    return '#A52175';
  }
}

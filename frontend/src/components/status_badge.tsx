import React, {FC} from 'react';
import styled from 'styled-components';

import {MarketStatus} from 'models/market';

interface StatusBadgeProps {
  status: MarketStatus;
  className?: string;
}

const StatusBadge: FC<StatusBadgeProps> = ({
  status,
  className,
}) => {
  const color = statusToColor(status);

  const Badge = styled.div`
    background-color: ${color};
    border-radius: 4px;

    color: white;
    font-weight: 700;
    text-align: center;
  `;
  return <Badge className={className}>{status}</Badge>;
};

export default StatusBadge;

export const Pc = styled(StatusBadge)`
  width: 87px;
  height: 27px;
  font-size: 15px;
  line-height: 27px;
`;

export const Mobile = styled(StatusBadge)`
  width: 72p;x
  height: 27px;
  font-size: 11px;
  line-height: 27px;
`;

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

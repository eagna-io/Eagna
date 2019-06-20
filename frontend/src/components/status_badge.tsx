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

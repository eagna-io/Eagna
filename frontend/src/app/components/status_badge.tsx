import React, {FC} from 'react';
import styled from 'styled-components';

import {pc} from 'app/components/responsive';
import {MarketStatus} from 'models/market';

interface StatusBadgeProps {
  status: MarketStatus;
  className?: string;
}

const StatusBadge: FC<StatusBadgeProps> = ({status, className}) => {
  const color = statusToColor(status);

  return (
    <Badge className={className} color={color}>
      {status}
    </Badge>
  );
};

export default StatusBadge;

const Badge = styled('div')<{color: string}>`
  width: 72p;
  height: 27px;
  line-height: 27px;
  font-size: 11px;
  background-color: ${props => props.color};
  border-radius: 4px;
  color: white;
  font-weight: 700;
  text-align: center;

  ${pc(`
    width: 87px;
    font-size: 15px;
  `)}
`;

function statusToColor(s: MarketStatus): string {
  if (s === 'Upcoming') {
    return '#D8D212';
  } else if (s === 'Open') {
    return '#23AC0E';
  } else if (s === 'Closed') {
    return '#3261AB';
  } else {
    return '#A52175';
  }
}

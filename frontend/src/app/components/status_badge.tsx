import React, { FC } from "react";
import styled from "styled-components";

import {
  Color,
  UpcomingMarketColor,
  OpenMarketColor,
  ClosedMarketColor,
  ResolvedMarketColor
} from "app/components/color";
import { pc } from "app/components/responsive";
import { MarketStatus } from "models/market";

interface StatusBadgeProps {
  status: MarketStatus;
  className?: string;
}

const StatusBadge: FC<StatusBadgeProps> = ({ status, className }) => {
  const color = statusToColor(status);

  return (
    <Badge className={className} bgcolor={color}>
      {status}
    </Badge>
  );
};

export default StatusBadge;

const Badge = styled("div")<{ bgcolor: Color }>`
  width: 72p;
  height: 27px;
  line-height: 27px;
  font-size: 11px;
  background-color: ${props => props.bgcolor.hex};
  border-radius: 4px;
  color: white;
  font-weight: 700;
  text-align: center;

  ${pc(`
    width: 87px;
    font-size: 15px;
  `)}
`;

function statusToColor(s: MarketStatus): Color {
  if (s === "Upcoming") {
    return UpcomingMarketColor;
  } else if (s === "Open") {
    return OpenMarketColor;
  } else if (s === "Closed") {
    return ClosedMarketColor;
  } else {
    return ResolvedMarketColor;
  }
}

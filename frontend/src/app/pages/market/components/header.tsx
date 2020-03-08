import React, { FC } from "react";
import styled from "styled-components";
import { Moment } from "moment";

import { pc, MinPcWidth } from "app/components/responsive";
import StatusBadge from "app/components/status_badge";

import { useMarket } from "./data_provider";

const MarketHeader: FC = () => {
  const { market } = useMarket();

  return (
    <Container>
      <StyledStatusBadge status={market.status} />
      <MarketPeriod open={market.attrs.open} close={market.attrs.close} />
      <MarketTitle>{market.attrs.title}</MarketTitle>
    </Container>
  );
};

export default MarketHeader;

const Container = styled.div`
  width: 100%;
  position: relative;
  padding: 25px 20px;
  background: #f6f8fa;
  box-shadow: 0 1px 0 0 #979797;

  ${pc(`
    height: 166px;
  `)}
`;

const StyledStatusBadge = styled(StatusBadge)`
  display: inline-block;
  width: 70px;
  height: 30px;
  font-size: 11px;
  line-height: 30px;

  ${pc(`
    position: absolute;
    top: 30px;
    right: calc((100% - ${MinPcWidth}px) / 2);
    width: 87px;
    height: 27px;
    font-size: 15px;
    font-weight: bold;
  `)}
`;

const MarketPeriod: FC<{ open: Moment; close: Moment }> = ({ open, close }) => {
  return (
    <MarketPeriodText>
      <MarketPeriodDate>{open.format("M/D")}</MarketPeriodDate>
      {open.format(" (ddd) HH:mm")}
      &nbsp;~&nbsp;
      <MarketPeriodDate>{close.format("M/D")}</MarketPeriodDate>
      {close.format(" (ddd) HH:mm")}
    </MarketPeriodText>
  );
};

const MarketPeriodText = styled.div`
  display: inline-block;
  width: calc(100% - 70px);
  text-align: right;
  font-size: 10px;

  ${pc(`
    position: absolute;
    bottom: 50px;
    right: calc((100% - ${MinPcWidth}px) / 2);
    font-size: 13px;
    font-weight: bold;
  `)}
`;

const MarketPeriodDate = styled.span`
  font-size: 12px;

  ${pc(`
    font-size: 15px;
  `)}
`;

const MarketTitle = styled.h3`
  margin: 0;
  margin-top: 20px;
  padding: 0;
  font-size: 20px;
  font-weight: bold;
  color: #0466d6;

  ${pc(`
    position: absolute;
    top: 30px;
    left: calc((100% - ${MinPcWidth}px) / 2);
    width: ${MinPcWidth - 220}px;
    margin-top: 0px;
    font-size: 28px;
  `)}
`;
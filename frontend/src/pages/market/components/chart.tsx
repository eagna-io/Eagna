import React, {FC} from 'react';
import styled from 'styled-components';
import {Chart} from 'react-google-charts';
import {Moment} from 'moment';

import {
  Token,
  PublicOrderHistory,
  TokenPrices,
  getTokenPrice,
  newTokenDistribution,
  addOrderToTokenDistribution,
  newTokenPrices,
} from 'models/market';

interface ChartComponentProps {
  tokens: Token[];
  orders: PublicOrderHistory;
  startTime: Moment;
  lmsrB: number;
  className?: string;
}

const ChartComponent: FC<ChartComponentProps> = ({
  tokens,
  orders,
  startTime,
  lmsrB,
  className,
}) => {
  const dataLabel = ['Day', ...tokens.map(t => t.name)];
  const data = tokenPricesHistory(tokens, orders, startTime, lmsrB).map(
    ([time, prices]) => [
      time.toDate(),
      ...tokens.map(t => getTokenPrice(prices, t.id)),
    ],
  );

  return (
    <Container className={className}>
      <Chart
        width="100%"
        height="100%"
        chartType="Line"
        loader={<div>Loading...</div>}
        data={[dataLabel, ...data]}
      />
    </Container>
  );
};

export default ChartComponent;

const Container = styled.div``;

function tokenPricesHistory(
  tokens: Token[],
  orders: PublicOrderHistory,
  startTime: Moment,
  lmsrB: number,
): (readonly [Moment, TokenPrices])[] {
  const distribution = newTokenDistribution(tokens);
  const priceHistory = [
    [startTime, newTokenPrices(lmsrB, distribution)] as const,
  ];

  orders.sort((a, b) => a.time.diff(b.time));
  orders.forEach(order => {
    addOrderToTokenDistribution(distribution, order);
    const prices = newTokenPrices(lmsrB, distribution);
    priceHistory.push([order.time, prices]);
  });

  return priceHistory;
}

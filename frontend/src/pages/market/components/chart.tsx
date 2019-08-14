import React, {FC} from 'react';
import styled from 'styled-components';
import Chart from 'react-apexcharts';
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
  const series = computeDataSeries(tokens, orders, startTime, lmsrB).map(
    ({token, data}) => ({
      name: token.name,
      data,
    }),
  );

  return (
    <Container className={className}>
      <Chart options={options} series={series} type="area" />
    </Container>
  );
};

export default ChartComponent;

const options = {
  chart: {
    height: '40%',
    stacked: false,
    zoom: {
      type: 'x',
      enabled: true,
    },
    toolbar: {
      show: false,
    },
  },
  plotOptions: {
    line: {
      curve: 'smooth',
    },
  },
  dataLabels: {
    enabled: false,
  },
  markers: {
    size: 0,
    style: 'full',
  },
  title: {
    show: false,
  },
  fill: {
    type: 'gradient',
    gradient: {
      shadeIntensity: 1,
      inverseColors: false,
      opacityFrom: 0.5,
      opacityTo: 0,
      stops: [0, 90, 100],
    },
  },
  grid: {
    show: false,
  },
  yaxis: {
    show: false,
  },
  xaxis: {
    type: 'datetime',
  },
  tooltip: {
    shared: false,
  },
};

const Container = styled.div`
  width: 100%;
`;

function computeDataSeries(
  tokens: Token[],
  orders: PublicOrderHistory,
  startTime: Moment,
  lmsrB: number,
): DataSeries {
  const distribution = newTokenDistribution(tokens);

  const series = initDataSeries(tokens);
  addData(series, startTime, newTokenPrices(lmsrB, distribution));

  orders.sort((a, b) => a.time.diff(b.time));

  orders.sort((a, b) => a.time.diff(b.time));
  orders.forEach(order => {
    addOrderToTokenDistribution(distribution, order);
    const prices = newTokenPrices(lmsrB, distribution);
    addData(series, order.time, prices);
  });

  return series;
}

type DataSeries = {token: Token; data: [Date, number][]}[];

function initDataSeries(tokens: Token[]): DataSeries {
  return tokens.map(token => ({
    token,
    data: [],
  }));
}

function addData(series: DataSeries, time: Moment, prices: TokenPrices) {
  series.forEach(({token, data}) => {
    const price = prices.get(token.id) as number;
    data.push([time.toDate(), price]);
  });
}

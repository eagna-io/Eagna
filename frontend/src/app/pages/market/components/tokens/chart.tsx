import React, { FC } from "react";
import styled from "styled-components";
import Chart from "react-apexcharts";

import { MarketToken } from "models/market";

import { useMarket } from "../data_provider";

interface Props {
  token: MarketToken;
}

const ChartComponent: FC<Props> = ({ token }) => {
  const { publicHistory } = useMarket();
  const priceHistory = publicHistory.price.get(token.name);
  const chartData = priceHistory.map(
    ({ date, price }) => [date.toDate(), price] as const
  );

  if (chartData.length <= 1) {
    return null;
  } else {
    const series = [
      {
        name: token.name,
        data: chartData
      }
    ];

    return (
      <Container>
        <Chart options={options} series={series} type="area" />
      </Container>
    );
  }
};

export default ChartComponent;

const Container = styled.div`
  width: 100%;
  margin-top: 30px;
  padding: 0px 22px;
`;

const options = {
  chart: {
    stacked: false,
    zoom: {
      type: "x",
      enabled: true
    },
    toolbar: {
      show: false
    }
  },
  plotOptions: {
    line: {
      curve: "smooth"
    }
  },
  dataLabels: {
    enabled: false
  },
  markers: {
    size: 0,
    style: "full"
  },
  title: {
    show: false
  },
  fill: {
    type: "gradient",
    gradient: {
      shadeIntensity: 1,
      inverseColors: false,
      opacityFrom: 0.5,
      opacityTo: 0,
      stops: [0, 90, 100]
    }
  },
  grid: {
    show: false
  },
  yaxis: {
    show: false
  },
  xaxis: {
    type: "datetime"
  },
  tooltip: {
    shared: false
  }
};
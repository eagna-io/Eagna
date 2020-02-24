import React from "react";
import styled from "styled-components";
import Chart from "react-apexcharts";
import moment from "moment";

import { Series, Tick } from "model/chart";

const Page: React.FC = () => {
  return <Chart options={options} series={series.apexchart} />;
};

export default Page;

const series = (() => {
  const series = new Series();
  series.push(new Tick(moment().subtract(1, "day"), 42));
  series.push(new Tick(moment(), 42));
  return series;
})();

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

import React from "react";
import { useSelector, useDispatch } from "react-redux";
import styled from "styled-components";
import Chart from "react-apexcharts";
import ApexCharts from "apexcharts";
import moment from "moment";

import { RootState } from "app/redux";
import { actions } from "app/redux/chart";
import { Record, lmsrPrice } from "model/chart";
import { DateTime, now } from "model/time";
import { Array, empty, normalizeSubtract } from "model/array";

type Data = [DateTime, number];
type Series = { name: string; data: Data[] }[];

const Page: React.FC = () => {
  const snapshot = useSelector((state: RootState) => state.chart.snapshot);
  const dispatch = useDispatch();

  React.useEffect(() => {
    let handler = setInterval(() => {
      if (Math.random() > 0.5) {
        dispatch(actions.vote({ outcome: "win", time: now() }));
      } else {
        dispatch(actions.vote({ outcome: "lose", time: now() }));
      }
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  React.useEffect(() => {
    const newWin = [
      snapshot.time,
      lmsrPrice(snapshot.distribution, "win")
    ] as Data;
    const newLose = [
      snapshot.time,
      lmsrPrice(snapshot.distribution, "Lose")
    ] as Data;

    ApexCharts.exec("the-chart", "appendData", [
      { data: [newWin] },
      { data: [newLose] }
    ]);
  }, [snapshot]);

  return (
    <Background>
      <ChartContainer>
        <Chart
          options={options}
          series={[
            { name: "win", data: [] },
            { name: "lose", data: [] }
          ]}
        />
      </ChartContainer>
    </Background>
  );
};

export default Page;

const Background = styled.div`
  width: 100vw;
  height: 100vh;
  padding: 30px;
  background-color: #121212;
`;

const ChartContainer = styled.div`
  background-color: #242423;
`;

const options = {
  colors: ["#bfe8ff", "#ffc0cb"],
  chart: {
    id: "the-chart",
    type: "line",
    foreColor: "#ffffff",
    stacked: false,
    toolbar: {
      show: false
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
  stroke: {
    width: 2,
    colors: ["#bfe8ff", "#ffc0cb"]
  },
  fill: {
    type: "gradient",
    gradient: {
      shadeIntensity: 1,
      inverseColors: false,
      opacityFrom: 0.7,
      opacityTo: 0.9,
      stops: [100, 90, 0]
    }
  },
  grid: {
    show: true,
    borderColor: "#555"
  },
  yaxis: {
    min: 0,
    max: 1000,
    tickAmount: 10,
    labels: {
      formatter: (val: number) => Math.floor(val)
    }
  },
  xaxis: {
    type: "datetime",
    labels: {
      formatter: (val: string, timestamp: number) => {
        return moment(val).format("M/D H:m");
      }
    },
    range: 1000 * 60
  },
  tooltip: {
    shared: false
  }
};

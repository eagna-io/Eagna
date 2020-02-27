import React from "react";
import { useSelector, useDispatch } from "react-redux";
import styled from "styled-components";
import Chart from "react-apexcharts";
import ApexCharts from "apexcharts";
import moment from "moment";
import { List } from "immutable";

import { RootState } from "app/redux";
import { actions } from "app/redux/chart";
import { Record } from "model/chart";

type Data = [number, number];
type Series = { name: string; data: Data[] }[];

const Page: React.FC = () => {
  const history = useSelector((state: RootState) => state.chart.history);
  const dispatch = useDispatch();
  const chartRef = React.useRef<List<Record>>(List());

  React.useEffect(() => {
    let handler = setInterval(() => {
      /*
      if (Math.random() > 0.5) {
        dispatch(actions.vote({ outcome: "win", time: moment() }));
      } else {
        dispatch(actions.vote({ outcome: "lose", time: moment() }));
      }
       */

      const price = Math.random() * 100 + 500;
      const now = Date.now();
      ApexCharts.exec("the-chart", "appendData", [
        { data: [[now, price]] },
        { data: [[now, 1000 - price]] }
      ]);
    }, 100);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  /*
  React.useEffect(() => {
    const prev = chartRef.current;
    const next = history;
    const updated = next.skip(prev.size);

    const updatedWin = updated
      .map(record => recordToData(record, "win"))
      .toArray();
    const updatedLose = updated
      .map(record => recordToData(record, "lose"))
      .toArray();

    chartRef.current = next;
    ApexCharts.exec("the-chart", "appendData", [
      { data: updatedWin },
      { data: updatedLose }
    ]);
  }, [history]);
   */

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

const recordToData = (record: Record, outcome: string): Data => {
  return [
    record.time.valueOf(),
    record.nextDistribution.lmsrPrice(outcome)
  ] as Data;
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
        return moment(timestamp).format("M/D H:m");
      }
    },
    range: 1000 * 30
  },
  tooltip: {
    shared: false
  }
};

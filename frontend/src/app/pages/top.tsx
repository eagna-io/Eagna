import React from "react";
import styled from "styled-components";
import Chart from "react-apexcharts";
import moment from "moment";
import produce from "immer";

type Data = [Date, number];
type Series = { name: string; data: Data[] }[];

const Page: React.FC = () => {
  const [series, setSeries] = React.useState<Series>([
    { name: "win", data: [] },
    { name: "lose", data: [] }
  ]);

  React.useEffect(() => {
    let handler = setInterval(() => {
      setSeries(computeNextSeries);
    }, 200);

    return () => {
      clearTimeout(handler);
    };
  }, []);

  return (
    <Background>
      <ChartContainer>
        <Chart options={options} series={series} />
      </ChartContainer>
    </Background>
  );
};

export default Page;

const computeNextSeries = (series: Series): Series => {
  const [win, lose] = series;
  const lastWinData =
    win.data.length === 0 ? undefined : win.data[win.data.length - 1];

  const nextWinData = computeNextWinData(lastWinData);
  const nextLoseData = [nextWinData[0], 1000 - nextWinData[1]] as Data;

  const nextWin = produce(win, last => {
    last.data.push(nextWinData);
  });
  const nextLose = produce(lose, last => {
    last.data.push(nextLoseData);
  });

  return [nextWin, nextLose];
};

const computeNextWinData = (last?: Data): Data => {
  if (!last) {
    return [moment().toDate(), 700];
  } else {
    const delta = Math.random() * 10 - 5;
    const nextVal = Math.max(Math.min(last[1] + delta, 998), 2);
    return [moment().toDate(), nextVal];
  }
};

const Background = styled.div`
  widt: 100vw;
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
    range: 1000 * 60 * 5
  },
  tooltip: {
    shared: false
  }
};

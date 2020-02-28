import React from "react";
import { useSelector, useDispatch } from "react-redux";
import styled from "styled-components";
import ApexCharts from "apexcharts";
import moment from "moment";

import { RootState } from "app/redux";
import { actions, ChartSnapshot } from "app/redux/chart";
import { lmsrPrice } from "model/chart";
import { DateTime, now } from "model/time";

type Data = [DateTime, number];
type Series = { name: string; data: Data[] }[];

const Page: React.FC = () => {
  const snapshot = useSelector((state: RootState) => state.chart.snapshot);
  const dispatch = useDispatch();
  const chartRef = React.useRef<ApexCharts | undefined>();
  const domRef = React.useRef<HTMLDivElement | null>(null);
  const snapshotRef = React.useRef<ChartSnapshot>(snapshot);

  React.useEffect(() => {
    let handler = setInterval(() => {
      if (Math.random() > 0.5) {
        dispatch(actions.vote({ outcome: "win", time: now() }));
      } else {
        dispatch(actions.vote({ outcome: "lose", time: now() }));
      }
    }, 100);

    return () => {
      clearInterval(handler);
    };
  }, [dispatch]);

  React.useEffect(() => {
    if (chartRef.current === undefined) {
      chartRef.current = new ApexCharts(domRef.current, options);
      chartRef.current.render();
    }
  }, [chartRef]);

  React.useEffect(() => {
    const handler = setInterval(() => {
      const snapshot = snapshotRef.current;
      ApexCharts.exec("the-chart", "appendData", [
        { data: [[snapshot.time, lmsrPrice(snapshot.distribution, "win")]] },
        { data: [[snapshot.time, lmsrPrice(snapshot.distribution, "lose")]] }
      ]);
    }, 211);

    return () => {
      clearInterval(handler);
    };
  }, [snapshotRef]);

  React.useEffect(() => {
    snapshotRef.current = snapshot;
  }, [snapshot]);

  return (
    <Background>
      <ChartContainer>
        <div ref={domRef} />
      </ChartContainer>
    </Background>
  );
};

export default Page;

const Background = styled.div`
  width: 100vw;
  height: 100vh;
  padding: 20px;
  background-color: #121212;
`;

const ChartContainer = styled.div`
  height: 400px;
  padding-top: 90px;
  background-color: #242423;
`;

const options = {
  colors: ["#bfe8ff", "#ffc0cb"],
  series: [
    {
      name: "win",
      data: []
    },
    {
      name: "lose",
      data: []
    }
  ],
  chart: {
    id: "the-chart",
    type: "line",
    height: "300",
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
  legend: {
    show: true,
    position: "top",
    fontSize: "10px",
    offsetY: 10,
    markers: {
      width: 8,
      height: 8
    }
  },
  title: {
    show: false
  },
  stroke: {
    width: 2,
    colors: ["#bfe8ff", "#ffc0cb"]
  },
  grid: {
    show: true,
    borderColor: "#555"
  },
  yaxis: {
    tickAmount: 5,
    labels: {
      style: {
        fontSize: "8px"
      },
      formatter: (val: number) => Math.floor(val)
    }
  },
  xaxis: {
    type: "datetime",
    labels: {
      rotate: 0,
      style: {
        fontSize: "8px"
      },
      formatter: (val: string, timestamp: number) => {
        return moment(val).format("HH:mm");
      }
    },
    axisBorder: {
      color: "#555"
    },
    axisTicks: {
      color: "#555"
    },
    range: 1000 * 60
  },
  tooltip: {
    shared: false
  }
};

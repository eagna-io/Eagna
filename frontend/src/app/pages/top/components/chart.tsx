import React from "react";
import { useSelector} from "react-redux";
import ApexCharts from "apexcharts";
import moment from "moment";

import { RootState } from "app/redux";
import { ChartSnapshot } from "app/redux/chart";
import { lmsrPrice } from "model/chart";

interface Props {
  height: number;
  renderInterval: number;
}

const Chart: React.FC<Props> = ({ height, renderInterval }) => {
  const snapshot = useSelector((state: RootState) => state.chart.snapshot);
  const chartRef = React.useRef<ApexCharts | undefined>();
  const domRef = React.useRef<HTMLDivElement | null>(null);
  const snapshotRef = React.useRef<ChartSnapshot>(snapshot);

  React.useEffect(() => {
    chartRef.current = new ApexCharts(domRef.current, createOptions(height));
    chartRef.current.render();
  }, [height]);

  React.useEffect(() => {
    const handler = setInterval(() => {
      const snapshot = snapshotRef.current;
      ApexCharts.exec("the-chart", "appendData", [
        { data: [[snapshot.time, lmsrPrice(snapshot.distribution, "win")]] },
        { data: [[snapshot.time, lmsrPrice(snapshot.distribution, "lose")]] }
      ]);
    }, renderInterval);

    return () => {
      clearInterval(handler);
    };
  }, [renderInterval]);

  React.useEffect(() => {
    snapshotRef.current = snapshot;
  }, [snapshot]);

  return <div ref={domRef} />;
};

export default Chart;

const createOptions = (height: number) => ({
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
    height: height,
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
    enabled: false,
    shared: false
  }
});

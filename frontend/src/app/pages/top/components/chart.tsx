import React from "react";
import { useSelector } from "react-redux";
import ApexCharts from "apexcharts";
import moment from "moment";

import { RootState } from "app/redux";
import { Data } from "app/redux/chart";
import { Map } from "model/map";

interface Props {
  height: number;
  renderInterval?: number;
}

const Chart: React.FC<Props> = ({ height, renderInterval = 100 }) => {
  const datasets = useSelector((state: RootState) => state.chart.datasets);
  const chartRef = React.useRef<ApexCharts | undefined>();
  const domRef = React.useRef<HTMLDivElement | null>(null);
  const datasetsRef = React.useRef<Map<Data[]>>(datasets);

  React.useEffect(() => {
    chartRef.current = new ApexCharts(domRef.current, createOptions(height));
    chartRef.current.render();
  }, [height]);

  React.useEffect(() => {
    const handler = setInterval(() => {
      const datasets = datasetsRef.current;
      chartRef.current!.updateSeries(
        [{ data: datasets.win }, { data: datasets.lose }],
        true
      );
    }, renderInterval);

    return () => {
      clearInterval(handler);
    };
  }, [renderInterval]);

  React.useEffect(() => {
    datasetsRef.current = datasets;
  }, [datasets]);

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
    animations: {
      dynamicAnimation: {
        enabled: false
      }
    },
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

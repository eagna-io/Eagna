import React from "react";
import ApexCharts from "apexcharts";
import moment from "moment";

import { Map } from "model/map";
import { WhiteBaseColor, ChartBaseColor, ChartGraphPink, ChartGraphBlue } from "app/components/color";

import { Data } from "../../reducer";

interface Props {
  height: string;
  renderInterval?: number;
  dataset: Data[];
}

const Chart: React.FC<Props> = ({ height, renderInterval = 100, dataset }) => {
  const chartRef = React.useRef<ApexCharts | undefined>();
  const domRef = React.useRef<HTMLDivElement | null>(null);
  const datasetsRef = React.useRef<Data[]>(dataset);

  // datasetを直接描画するのではなく、refに入れて定期的に描画する
  datasetsRef.current = dataset;

  // DOMの描画が終わった後に呼ばれるので、domRef.currentに値が入っている
  React.useEffect(() => {
    chartRef.current = new ApexCharts(domRef.current, createOptions(height));
    chartRef.current.render();
  }, [height]);

  // 定期的に描画
  React.useEffect(() => {
    const handler = setInterval(() => {
      const dataset = datasetsRef.current;
      chartRef.current!.updateSeries(
        [{ data: dataset }],
        true
      );
    }, renderInterval);

    return () => {
      clearInterval(handler);
    };
  }, [renderInterval]);

  return <div ref={domRef} />;
};

export default Chart;

const createOptions = (height: string) => ({
  colors: [ChartGraphBlue.hex, ChartGraphPink.hex],
  series: [
    {
      name: "勝利する",
      data: []
    }
  ],
  chart: {
    id: "the-chart",
    type: "line",
    height: height,
    foreColor: WhiteBaseColor.hex,
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
    colors: [ChartGraphBlue.hex]
  },
  grid: {
    show: true,
    borderColor: ChartBaseColor.hex
  },
  yaxis: {
    opposite: true,
    tickAmount: 5,
    labels: {
      style: {
        fontSize: "8px",
        colors: ChartBaseColor.hex,
      },
      formatter: (val: number) => Math.floor(val)
    },
  },
  xaxis: {
    type: "datetime",
    labels: {
      rotate: 0,
      style: {
        fontSize: "8px",
        colors: ChartBaseColor.hex,
      },
      formatter: (val: string, timestamp: number) => {
        return moment(val).format("HH:mm");
      }
    },
    axisBorder: {
      color: ChartBaseColor.hex
    },
    axisTicks: {
      color: ChartBaseColor.hex
    },
    range: 1000 * 60
  },
  tooltip: {
    enabled: false,
    shared: false
  }
});

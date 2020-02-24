import { Moment } from "moment";

export class Series {
  readonly ticks: Tick[];
  readonly apexchart: { data: ApexchartTick[] }[];

  constructor() {
    this.ticks = [];
    this.apexchart = [{data: []}];
  }

  push(tick: Tick) {
    this.ticks.push(tick);
    this.apexchart[0].data.push([tick.time.unix(), tick.price]);
  }
}

export type ApexchartTick = [number, number];

export class Tick {
  constructor(readonly time: Moment, readonly price: number) {}
}

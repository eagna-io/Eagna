import { Moment } from "moment";
import produce from "immer";

export class BinaryChart {
  private winRecords: Record[];
  private loseRecords: Record[];
  private apexWinRecords: ApexRecord[];
  private apexLoseRecords: ApexRecord[];

  constructor() {
    this.winRecords = [];
    this.loseRecords = [];
    this.apexWinRecords = [];
    this.apexLoseRecords = [];
  }

  addWin(winRecord: Record): BinaryChart {
    const loseRecord = new Record(winRecord.time, 1000 - winRecord.price);
    return this.addRecord(winRecord, loseRecord);
  }

  addLose(loseRecord: Record): BinaryChart {
    const winRecord = new Record(loseRecord.time, 1000 - loseRecord.price);
    return this.addRecord(winRecord, loseRecord);
  }

  private addRecord(winRecord: Record, loseRecord: Record): BinaryChart {
    const nextWinRecords = produce(this.winRecords, records => {
      records.push(winRecord);
    });
    const nextLoseRecords = produce(this.loseRecords, records => {
      records.push(loseRecord);
    });
    const nextApexWinRecords = produce(this.apexWinRecords, records => {
      records.push(winRecord.toApexRecord());
    });
    const nextApexLoseRecords = produce(this.apexLoseRecords, records => {
      records.push(loseRecord.toApexRecord());
    });
    const newChart = new BinaryChart();
    newChart.winRecords = nextWinRecords;
    newChart.loseRecords = nextLoseRecords;
    newChart.apexWinRecords = nextApexWinRecords;
    newChart.apexLoseRecords = nextApexLoseRecords;
    return newChart;
  }

  getApexRecords(): { name: string; data: ApexRecord[] }[] {
    return [
      { name: "win", data: this.apexWinRecords },
      { name: "lose", data: this.apexLoseRecords }
    ];
  }
}

export type ApexRecord = [Date, number];

export class Record {
  constructor(readonly time: Moment, readonly price: number) {}

  toApexRecord(): ApexRecord {
    return [this.time.toDate(), this.price] as ApexRecord;
  }
}

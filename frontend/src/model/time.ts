import moment from "moment";

export type DateTime = string;

export const now = (): DateTime => moment().toISOString();

export const parse = (s: string): DateTime => moment(s).toISOString();

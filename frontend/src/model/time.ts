import moment from "moment";

// Milli seconds Timestamp
export type DateTime = number;

export const now = (): DateTime => moment().valueOf();

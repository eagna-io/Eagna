import { Moment } from "moment";

export interface Poll {
  id: string;
  idx: number;
  title: string;
  endAt: Moment;
  status?: "open" | "closed";
  choices: Record<string, string>;
  resolved?: string;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
  selected?: string;
}

export interface Comment {
  account: string;
  comment: string;
  color: string;
}

export type Timer = number | "closed";

import { Moment } from "moment";

export interface Poll {
  id: string;
  title: string;
  endAt: Moment;
  status: "open" | "closed";
  choices: Record<string, string>;
  resolved?: string;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
}

export interface Comment {
  account: string;
  comment: string;
  color: string;
}

export type Timer = number | "closed" | "correct" | "incorrect";

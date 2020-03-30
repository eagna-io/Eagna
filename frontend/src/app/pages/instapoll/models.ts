import { Moment } from "moment";

export interface Comment {
  account: string;
  comment: string;
  color: string;
}

export interface Poll {
  id: string;
  endAt: Moment;
  status: "open" | "closed";
  choices: Record<string, string>;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
}

export type Timer = number | "closed" | "correct" | "incorrect";

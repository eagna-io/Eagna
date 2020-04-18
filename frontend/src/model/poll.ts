import { Moment } from "moment";

export interface Poll {
  id: string;
  idx: number;
  status: "Open" | "Closed";
  title: string;
  choices: {
    name: string;
    color: string;
    idx: number;
  }[];
  created_at: Moment,
  duration_sec: number,
  resolved_choice?: string;
  stats?: {
    totalVotes: number;
    votePerChoice: Record<string, number>;
  };
  selected?: string; // selected_choice
}

export interface Comment {
  account_name: string;
  comment: string;
  choice?: string;
}

export type Timer = number | "closed";

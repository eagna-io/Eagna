import { Moment } from "moment";

export interface Contest {
  id: string;
  title: string;
  status: "Upcoming" | "Open" | "Closed" | "Archived";
  category: string;
  event_start_at?: Moment;
}

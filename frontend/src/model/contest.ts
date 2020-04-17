export interface Contest {
  category: string;
  title: string;
  startAt: string;
  status: "upcoming" | "open" | "closed" | "archived";
}

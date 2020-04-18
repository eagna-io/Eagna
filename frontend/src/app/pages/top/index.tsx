import React from "react";
import moment from "moment";

import * as contestApi from "infra/http/contest";
import { Contest } from "model/contest";

import { Page } from "./page";

export const Top: React.FC = () => {
  const [contests, setContests] = React.useState<Contest[]>([]);

  React.useEffect(() => {
    contestApi.getList().then(res => setContests(res.contests));
  }, []);

  return <Page contests={contests} />;
};

const demoContests = [
  {
    id: "1",
    title: "Los Angels Lakers vs Golden State Warriors",
    status: "Upcoming" as const,
    category: "NBA（バスケ）",
    event_start_at: moment()
  },
  {
    id: "2",
    title: "Los Angels Lakers vs Golden State Warriors",
    status: "Open" as const,
    category: "NBA（バスケ）",
    event_start_at: moment()
  }
];

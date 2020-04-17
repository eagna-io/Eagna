import React from "react";

import { Page } from "./page";

export const Top: React.FC = () => {
  return (
    <Page contests={contest}/>
  );
}

const contest = [
  {
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    startAt: "2020.06.01｜11:00",
    status: "upcoming" as const
  },
  {
    category: "NBA（バスケ）",
    title: "Los Angels Lakers vs Golden State Warriors",
    startAt: "2020.06.01｜11:00",
    status: "open" as const
  },
];

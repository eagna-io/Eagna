import React from "react";

import { Page } from "./page";

export const Top: React.FC = () => {
  return (
    <Page contest={contest}/>
  );
}

const contest = {
  category: "NBA（バスケ）",
  title: "Los Angels Lakers vs Golden State Warriors",
  startAt: "2020.06.01｜11:00",
  status: "upcoming" as const
}

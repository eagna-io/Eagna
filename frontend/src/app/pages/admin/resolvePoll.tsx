import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Poll } from "model/poll";
import { Contest } from "model/contest";
import * as pollApi from "infra/http/poll";
import * as storage from "infra/storage";

import { AdminTemplate } from "./components/template/admin";
import { ResolveItem } from "./components/organisms/resolveItem";
import { ContestIdInput } from "./components/organisms/contestIdInput";

export const ResolvePoll: React.FC = () => {
  const [accessToken] = React.useState(storage.getAdminAccessToken);
  const [contest, setContest] = React.useState<Contest | undefined>();
  const [poll, setPoll] = React.useState<Poll | undefined>();

  const onResolve = (choice: string) => {
    if (!accessToken) {
      alert("ログインが必要です");
      return;
    }
    if (poll && contest) {
      const confirmed = window.confirm(
        `「${poll.title}」を「${choice}」でResolveします。よろしいですか？`
      );
      if (confirmed) {
        pollApi
          .resolve({
            contestId: contest.id,
            pollId: poll.id,
            choice,
            accessToken
          })
          .then(res => alert("Resolveしました"));
      }
    }
  };

  return (
    <AdminTemplate>
      <ContestIdInput
        onFetchContest={(contest) => {
          setContest(contest);
          const poll = [...contest.polls]
          .sort((a, b) => b.idx - a.idx) // DESC
          .find(
            poll =>
              poll.status === "Closed" &&
              poll.resolved_choice === undefined
          );
          if (poll) {
            setPoll(poll);
          }
        }}
      />
      {contest ? <PollTitle>{contest.title}</PollTitle> : null}
      {poll ? <PollTitle>{poll.title}</PollTitle> : null}
      {poll &&
      poll.status === "Closed" &&
      poll.resolved_choice === undefined ? (
        <ResolveContainer>
          {poll.choices.map(({ name }) => (
            <ResolveItem key={name} choiceItem={name} onResolve={onResolve} />
          ))}
        </ResolveContainer>
      ) : null}
    </AdminTemplate>
  );
};

const PollTitle = styled.div`
  font-size: 21px;
  font-weight: 500;
  color: ${color.AdminMainColor.hex};
`;

const ResolveContainer = styled.div`
  margin: 70px 0px 0px 95px;
`;

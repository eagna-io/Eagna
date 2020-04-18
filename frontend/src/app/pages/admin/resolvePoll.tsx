import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Poll } from "model/poll";
import { Contest } from "model/contest";
import * as pollApi from "infra/http/poll";
import * as contestApi from "infra/http/contest";
import * as storage from "infra/storage";

import { AdminTemplate } from "./components/template/admin";
import { ResolveItem } from "./components/organisms/resolveItem";

export const ResolvePoll: React.FC = () => {
  const [accessToken] = React.useState(storage.getAdminAccessToken);
  const [contestId, setContestId] = React.useState("");
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
      <ContestIdContainer>
        <ContestIdTag>コンテストId</ContestIdTag>
        <ContestIdInput onChange={e => setContestId(e.target.value)} />
        <ContestSearchButton
          onClick={() => {
            if (contestId) {
              contestApi
                .get(contestId)
                .then(res => {
                  setContest(res);
                  const poll = [...res.polls]
                    .sort((a, b) => b.idx - a.idx) // DESC
                    .find(
                      poll =>
                        poll.status === "Closed" &&
                        poll.resolved_choice === undefined
                    );
                  if (poll) {
                    setPoll(poll);
                  }
                })
                .catch(e => {
                  console.error(e);
                  alert(`Error ${JSON.stringify(e)}`);
                });
            }
          }}
        >
          検索する
        </ContestSearchButton>
      </ContestIdContainer>
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

const ContestIdContainer = styled.div`
  margin-bottom: 50px;
`;

const ContestIdTag = styled.span`
  width: 150px;
  font-size: 14px;
  color: ${color.AdminMainColor.hex};
`;

const ContestIdInput = styled.input`
  width: 250px;
  height: 30px;
  padding: 10px 8px;
  margin-left: 50px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
`;

const ContestSearchButton = styled.button`
  width: 100px;
  height: 30px;
  vertical-align: bottom;
  margin-left: 50px;
  background-color: ${color.AdminMainColor.hex};
  color: white;
`;

const PollTitle = styled.div`
  font-size: 21px;
  font-weight: 500;
  color: ${color.AdminMainColor.hex};
`;

const ResolveContainer = styled.div`
  margin: 70px 0px 0px 95px;
`;

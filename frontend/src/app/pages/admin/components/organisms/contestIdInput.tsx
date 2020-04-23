import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import * as contestApi from "infra/http/contest";
import { Contest } from "model/contest";
import { Poll } from "model/poll";

interface Props {
  onFetched: (contest: Contest, poll: Poll | undefined) => void;
}

export const ContestIdInput: React.FC<Props> = ({ onFetched }) => {
  const [contestId, setContestId] = React.useState("");
  return (
    <Container>
      <IdTag>コンテストId</IdTag>
      <IdInput onChange={e => setContestId(e.target.value)} />
      <SearchButton
        onClick={() => {
          if (contestId) {
            contestApi
              .get(contestId)
              .then(res => {
                // setContest(res);
                const poll = [...res.polls]
                  .sort((a, b) => b.idx - a.idx) // DESC
                  .find(
                    poll =>
                      poll.status === "Closed" &&
                      poll.resolved_choice === undefined
                  );
                onFetched(res, poll)
              })
              .catch(e => {
                console.error(e);
                alert(`Error ${JSON.stringify(e)}`);
              });
          }
        }}
      >
        検索する
      </SearchButton>
    </Container>
  );
};
const Container = styled.div`
  margin-bottom: 50px;
`;

const IdTag = styled.span`
  width: 150px;
  font-size: 14px;
  color: ${color.AdminMainColor.hex};
`;

const IdInput = styled.input`
  width: 250px;
  height: 30px;
  padding: 10px 8px;
  margin-left: 50px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
`;

const SearchButton = styled.button`
  width: 100px;
  height: 30px;
  vertical-align: bottom;
  margin-left: 50px;
  background-color: ${color.AdminMainColor.hex};
  color: white;
`;


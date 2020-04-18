import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Poll } from "model/poll";
import * as pollApi from "infra/http/poll";

import { AdminTemplate } from "./components/template/admin";
import { ResolveItem } from "./components/organisms/resolveItem";

export const ResolvePoll: React.FC = () => {
  const [poll, setPoll] = React.useState<Poll | undefined>();

  React.useEffect(() => {
    pollApi.get().then(poll => setPoll(poll));
  }, []);

  if (!poll) {
    return <AdminTemplate>Pollが見つかりません</AdminTemplate>;
  } else if (poll.status === "Open" || poll.resolved_choice) {
    return <AdminTemplate>Pollが見つかりません</AdminTemplate>;
  } else {
    return (
      <AdminTemplate>
        <PollTitle>{poll.title}</PollTitle>
        <ResolveContainer>
          {Object.keys(poll.choices).map(choice => (
            <ResolveItem
              key={choice}
              choiceItem={choice}
              pollTitle={poll.title}
            />
          ))}
        </ResolveContainer>
      </AdminTemplate>
    );
  }
};

const PollTitle = styled.div`
  font-size: 21px;
  font-weight: 500;
  color: ${color.AdminMainColor.hex};
`;

const ResolveContainer = styled.div`
  margin: 70px 0px 0px 95px;
`;

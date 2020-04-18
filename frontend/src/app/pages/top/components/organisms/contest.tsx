import React from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import * as color from "app/components/color";
import { Contest } from "model/contest";

import { StartSchedule } from "../molecules/startSchedule";
import { OnGoing } from "../molecules/onGoing";

interface Props {
  contest: Contest;
}

export const ContestComponent: React.FC<Props> = ({ contest }) => {
  if (contest.status === "Upcoming" || contest.status === "Open") {
    return (
      <Container>
        <Link to={`/contest/${contest.id}`}>
          <ContestImage />
          <ContestDetail>
            <Category>{contest.category}</Category>
            <Title>{contest.title}</Title>
            {contest.status === "Upcoming" ? (
              <StartSchedule startAt={contest.event_start_at} />
            ) : contest.status === "Open" ? (
              <OnGoing />
            ) : null}
          </ContestDetail>
        </Link>
      </Container>
    );
  } else {
    return null;
  }
};

const Container = styled.div`
  width: 100%;
  margin-bottom: 26px;
  border-radius: 4px;
  box-shadow: 2px 2px 4px 0 ${color.BlackColor.rgba(0.5)};
`;

const ContestImage = styled.div`
  height: 132px;
  background-image: url("/img/top/contest-image.png");
  background-position: right 50% bottom 50%;
  background-size: cover;
  border-radius: 4px 4px 0 0;
`;

const ContestDetail = styled.div`
  padding: 16px 16px 10px 16px;
  background: ${color.WhiteBaseColor.hex};
  border-radius: 0 0 4px 4px;
`;

const Category = styled.div`
  font-size: 16px;
  font-weight: 800;
  color: ${color.TextBaseColor.hex};
  margin-bottom: 12px;
`;

const Title = styled.div`
  font-size: 14px;
  font-weight: 600;
  color: ${color.TextBaseColor.hex};
  margin-bottom: 12px;
`;

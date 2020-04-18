import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Contest } from "model/contest";

import { StartSchedule } from "../molecules/startSchedule";
import { OnGoing } from "../molecules/onGoing";
import ContestBackGroundImage from "../atoms/images/contest-image.png";

interface Props {
  contest: Contest;
}

export const ContestComponent: React.FC<Props> = ({ contest }) => {
  if (contest.status === "upcoming" || contest.status === "open") {
    return (
      <Container>
        <ContestImage />
        <ContestDetail>
          <Category>{contest.category}</Category>
          <Title>{contest.title}</Title>
          { contest.status === "upcoming" ? <StartSchedule startAt={contest.startAt} /> : contest.status === "open" ? <OnGoing /> : null }
        </ContestDetail>
      </Container>
    );
  } else {
    return null
  }
}

console.log(ContestBackGroundImage);

const Container = styled.div`
  width: 100%;
  margin-bottom: 26px;
  border-radius: 4px;
  box-shadow: 2px 2px 4px 0 ${color.BlackColor.rgba(0.5)};
`;

const ContestImage = styled.div`
  height: 132px;
  backgroundImage: url(${ContestBackGroundImage});
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


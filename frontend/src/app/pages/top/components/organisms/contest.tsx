import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";

import { ReactComponent as FireIcon } from "./components/atoms/images/fire.svg";
import { ReactComponent as TimerBlackIcon } from "../atoms/images/clock-b.svg";

interface Props {
  category: string;
  title: string;
  startAt: string;
}

export const Contest: React.FC<Props> = ({ category, title, startAt }) => {
  return (
    <Container>
      <ContestImage></ContestImage>
      <ContestDetail>
        <Category>{category}</Category>
        <Title>{title}</Title>
        <StartAt>
          <Timer />
          <Date>{startAt}</Date>
        </StartAt>
      </ContestDetail>
    </Container>
  );
}

const Container = styled.div`
  width: 100%;
  margin-bottom: 26px;
  border-radius: 4px;
  box-shadow: 2px 2px 4px 0 ${color.BlackColor.rgba(0.5)};
`;

const ContestImage = styled.div`
  height: 132px;
  background: ${color.TextBaseColor.hex};
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

const StartAt = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: center;
  margin-bottom: 12px;
`;

const Timer = styled(TimerBlackIcon)`
  width: 14px;
  height: 14px;
  margin-right: 4px;
`;

const Date = styled.div`
  font-size: 12px;
  color: ${color.TextBaseColor.hex};
`;
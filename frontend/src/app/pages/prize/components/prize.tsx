import React from "react";
import styled from "styled-components";
import { Prize } from "models/prize";

interface Props {
  prize: Prize;
}

export default ({ prize }: Props) => {
  return (
    <Container>
      <Thumbnail src={prize.thumbnailUrl} />
      <Contents>
        <Name>{prize.name}</Name>
        <Point>{prize.price} ポイント</Point>
      </Contents>
    </Container>
  );
};

const Container = styled.div`
  display: inline-block;
  width: 45vw;
  margin-top: 40px;
  border-radius: 4px;
  box-shadow: 0 0 2px 0 rgba(0, 0, 0, 0.5);
  overflow: hidden;

  &:nth-of-type(even) {
    margin-left: 13px;
  }
`;

const Thumbnail = styled("div")<{ src: string }>`
  width: 100%;
  height: 170px;
  background-image: url(${props => props.src});
  background-size: cover;
  background-position: center;
`;

const Contents = styled.div`
  width: 100%;
  padding: 6px;
`;

const Name = styled.h5`
  margin: 0;
  font-size: 12px;
  font-weight: bold;
`;

const Point = styled.p`
  margin-top: 15px;
  font-size: 14px;
  font-weight: bold;
  color: #f9aa33;
  text-align: right;
`;

import React from "react";
import styled from "styled-components";
import { useSelector } from "react-redux";
import { RootState } from "app/redux";

export default () => {
  const user = useSelector((state: RootState) => state.user.user);
  return (
    <Container>
      <MainContainer>
        <Title>保有ポイント</Title>
        <Icon src="/img/prize/trophy.svg" />
        <Point>
          {user ? user.point : "-"}
          <PointUnit> ポイント</PointUnit>
        </Point>
      </MainContainer>
      <BottomContainer>ポイント獲得・使用履歴を確認する</BottomContainer>
    </Container>
  );
};

const Container = styled.div`
  position: relative;
  width: 100%;
  max-width: 500px;
  top: 30px;
  border-radius: 4px;
  border: solid 1px #d1d5da;
  background-color: white;
`;

const MainContainer = styled.div`
  width: 100%;
  padding: 10px;
`;

const Title = styled.h3`
  width: 100%;
  margin: 0;
  font-size: 14px;
  font-weight: bold;
`;

const Icon = styled.img`
  display: inline-block;
  margin-top: 20px;
  margin-left: 30px;
  width: 40px;
  height: 40px;
`;

const Point = styled.div`
  display: inline-block;
  position: absolute;
  right: 30px;
  top: 60px;
  font-size: 21px;
  font-weight: bold;
`;

const PointUnit = styled.span`
  font-size: 18px;
`;

const BottomContainer = styled.div`
  width: 100%;
  padding: 6px 11px;
  border-top: solid 1px #d1d5da;
  font-size: 10px;
  font-weight: thin;
`;

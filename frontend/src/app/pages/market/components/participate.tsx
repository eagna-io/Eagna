import React, { FC } from "react";
import styled from "styled-components";
import { useSelector } from 'react-redux';

import { User } from "models/user";
import { MarketStatus } from "models/market";
import { Order, OrderRepository } from "models/order";
import { RootState } from 'app/redux';

import { useMarket } from "./data_provider";

export default () => {
  const user = useSelector((state: RootState) => state.user.user);

  const { market } = useMarket();

  if (market.status === MarketStatus.Open) {
    if (user instanceof User) {
      return <RequestParticipateComponent user={user} />;
    }
  }
  return null;
};

const RequestParticipateComponent: React.FC<{ user: User }> = ({ user }) => {
  const { market, updateMarket } = useMarket();
  const [requesting, setRequesting] = React.useState(false);

  const requestParticipate = async () => {
    setRequesting(true);
    await OrderRepository.create(market, user, Order.coinSupply());
    updateMarket();
    setRequesting(false);
  };

  return (
    <Container>
      <ParticipateButton disabled={requesting} onClick={requestParticipate}>
        参加する
      </ParticipateButton>
    </Container>
  );
};

const Container = styled.div`
  width: 100%;
  padding: 50px 0;
`;

const ParticipateButton = styled.button`
  display: block;
  width: 100px;
  height: 40px;
  margin: 0 auto;
  background-color: #1c384e;
  box-shadow: 1px 1px 2px 0 rgba(0, 0, 0, 0.5);
  border: none;
  font-size: 15px;
  font-weight: bold;
  color: white;
  text-align: center;

  &:disabled {
    background-color: #9b9b9b;
  }
`;

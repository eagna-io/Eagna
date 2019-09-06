import React, { FC } from "react";
import styled from "styled-components";
import { Link, RouteComponentProps, withRouter } from "react-router-dom";

import { User } from "models/user";
import { MarketStatus } from "models/market";
import { Order, OrderRepository } from "models/order";
import { withUser, UserProps } from "app/components/user";
import { pc } from "app/components/responsive";

import { useMarket } from "./data_provider";

const ParticipateComponent: FC<RouteComponentProps & UserProps> = ({
  user,
  location
}) => {
  const { market } = useMarket();
  if (market.status === MarketStatus.Open) {
    if (user === "Checking") {
      return <LoginCheckingComponent />;
    } else if (user === null) {
      return <NeedLoginComponent here={location.pathname} />;
    } else {
      return <RequestParticipateComponent user={user} />;
    }
  } else if (market.status === MarketStatus.Upcoming) {
    return <MarketIsUpcomingComponent />;
  } else {
    return <MarketIsClosedComponent />;
  }
};

export default withRouter(withUser(ParticipateComponent));

const LoginCheckingComponent: React.FC = () => (
  <Container>
    <CheckingDialog>Checking...</CheckingDialog>
    <Message>取引を行うにはログインする必要があります。</Message>
  </Container>
);

const NeedLoginComponent: React.FC<{ here: string }> = ({ here }) => (
  <Container>
    <LoginButton
      to={{
        pathname: "/login",
        state: { redirect: here }
      }}
    >
      ログイン
    </LoginButton>
    <Message>取引を行うにはログインする必要があります。</Message>
  </Container>
);

const RequestParticipateComponent: React.FC<{ user: User }> = ({ user }) => {
  const { market } = useMarket();
  const requestParticipate = async () => {
    await OrderRepository.create(market, user, Order.coinSupply());
    // TODO
    window.location.reload();
  };
  return (
    <Container>
      <ParticipateButton onClick={requestParticipate}>
        参加する
      </ParticipateButton>
      <Message>
        「参加する」ボタンを押すと、コインが配布され取引をできるようになります
      </Message>
    </Container>
  );
};

const MarketIsUpcomingComponent: React.FC = () => (
  <Container>
    <InactiveParticipateButton>参加する</InactiveParticipateButton>
    <Message>
      マーケットがOpen状態になると、「参加する」ボタンが押せるようになります。
    </Message>
  </Container>
);

const MarketIsClosedComponent: React.FC = () => (
  <Container>
    <InactiveParticipateButton>参加する</InactiveParticipateButton>
    <Message>マーケットはすでに閉じています。</Message>
  </Container>
);

const Container = styled.div`
  width: 100%;
  padding: 50px 0;
`;

const CheckingDialog = styled.div`
  display: inline-block;
  width: 100px;
  height: 40px;
  background-color: #1c384e;
  box-shadow: 1px 1px 2px 0 rgba(0, 0, 0, 0.5);
  border: none;
  font-size: 15px;
  font-weight: bold;
  line-height: 40px;
  color: white;
  text-align: center;
`;

const ParticipateButton = styled.button`
  width: 100px;
  height: 40px;
  background-color: #1c384e;
  box-shadow: 1px 1px 2px 0 rgba(0, 0, 0, 0.5);
  border: none;
  font-size: 15px;
  font-weight: bold;
  color: white;
  text-align: center;
`;

const InactiveParticipateButton = styled(ParticipateButton)`
  background-color: #9b9b9b;
  color: white;
`;

const LoginButton = styled(Link)`
  display: inline-block;
  width: 100px;
  height: 40px;
  background-color: #1c384e;
  box-shadow: 1px 1px 2px 0 rgba(0, 0, 0, 0.5);
  border: none;
  font-size: 15px;
  font-weight: bold;
  color: white;
  text-align: center;
  line-height: 40px;

  &:visited {
    color: white;
  }
`;

const Message = styled.p`
  display: inline-block;
  width: calc(100% - 100px - 30px);
  margin-left: 30px;
  font-size: 10px;
  font-weight: bold;
  vertical-align: top;

  ${pc(`
    font-size: 14px;
    vertical-align: baseline;
  `)}
`;

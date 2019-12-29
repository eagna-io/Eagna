import React, { FC, useState, useEffect } from "react";
import styled from "styled-components";
import { withRouter } from "react-router-dom";
import { History } from "history";
import ReactGA from "react-ga";
import { useSelector } from "react-redux";

import { Market, MarketStatus, MarketRepository } from "models/market";
import { User } from "models/user";
import { RootState } from "app/redux";
import { MinPcWidth } from "app/components/responsive";
import Header from "app/components/header";

import ProfileComponent from "./components/profile";
import MarketsComponent from "./components/markets";
import NewsComponent from "./components/news";

interface AccountPageProps {
  history: History;
}

const AccountPageWrapper: FC<AccountPageProps> = ({ history }) => {
  const user = useSelector((state: RootState) => state.user.user);

  useEffect(() => {
    ReactGA.pageview("/account");
  }, []);

  useEffect(() => {
    if (user === null) {
      history.push("/signin");
    }
  });

  if (user === null || user === undefined) {
    return <h2>Loading...</h2>;
  } else {
    return <AccountPage user={user} />;
  }
};

export default withRouter(AccountPageWrapper);

const AccountPage: FC<{ user: User }> = ({ user }) => {
  const [participatedMarkets, setParticipatedMarkets] = useState<Market[]>([]);
  const [featuredMarkets, setFeaturedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    (async () => {
      const markets = await MarketRepository.queryListOfMine(user);
      setParticipatedMarkets(markets.map(({ market }) => market));
    })();
    (async () => {
      const markets = await MarketRepository.queryListOfStatus([
        MarketStatus.Upcoming,
        MarketStatus.Open
      ]);
      setFeaturedMarkets(markets.map(({ market }) => market));
    })();
  }, [user]);

  return (
    <>
      <Header />
      <Container>
        <ProfileComponent user={user} />
        <MarketsComponent
          title="参加している/参加したマーケット"
          markets={participatedMarkets}
        />
        <MarketsComponent title="注目のマーケット" markets={featuredMarkets} />
        <NewsComponent />
      </Container>
    </>
  );
};

const Container = styled.div`
  width: 95%;
  max-width: ${MinPcWidth}px;
  margin: 0 auto;
  padding: 50px 0;
`;

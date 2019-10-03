import React, { FC } from "react";
import styled from "styled-components";
import ReactGA from "react-ga";
import { useSelector } from "react-redux";

import { User } from "models/user";
import { RootState } from "app/redux";
import { MinPcWidth } from "app/components/responsive";
import Header from "app/components/header";
import NotFoundPage from "app/pages/not_found";

import MarketDataProvider, { useMarket } from "./components/data_provider";
import MessageComponent from "./components/message";
import MarketHeader from "./components/header";
import ParticipateComponent from "./components/participate";
import TokenListComponent from "./components/tokens";
import OrganizerComponent from "./components/organizer";
import CoinsComponent from "./components/coins";
import PrizeComponent from "./components/prize";
import DescComponent from "./components/description";

interface MarketPageProps {
  marketId: string;
}

const MarketPage: FC<MarketPageProps> = ({ marketId }) => {
  const user = useSelector((state: RootState) => state.user.user);

  React.useEffect(() => {
    ReactGA.pageview(`/market/${marketId}`);
  }, [marketId]);

  return (
    <MarketDataProvider
      marketId={marketId}
      user={user instanceof User ? user : null}
      loadingView={<LoadingView />}
      notfoundView={<NotFoundPage />}
    >
      <MarketPageInner />
    </MarketDataProvider>
  );
};

export default MarketPage;

const MarketPageInner: FC = () => {
  const { myHistory } = useMarket();
  return (
    <>
      <Header />
      <MessageComponent />
      <MarketHeader />
      <Contents>
        {!myHistory ? (
          <>
            <ParticipateComponent />
            <HR />
          </>
        ) : null}
        <CoinsComponent />
        <TokenListComponent />
        <HR />
        <OrganizerComponent />
        <HR />
        <PrizeComponent />
        <HR />
        <DescComponent />
      </Contents>
    </>
  );
};

const LoadingView: FC = () => (
  <>
    <Header />
    <h2>Loading...</h2>
  </>
);

const Contents = styled.div`
  width: 90%;
  max-width: ${MinPcWidth}px;
  margin: 0 auto;
`;

const HR = styled.hr`
  border: 0.5px solid #979797;
  margin: 0;
`;
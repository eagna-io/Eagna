import React, { FC } from "react";
import styled from "styled-components";
import ReactGA from "react-ga";
import { useSelector } from "react-redux";

import { User } from "models/user";
import { RootState } from "app/redux";
import { MinPcWidth } from "app/components/responsive";
import Header from "app/components/header";
import NotFoundPage from "app/pages/not_found";

import MarketDataProvider from "./components/data_provider";
import MessageComponent from "./components/message";
import MarketHeader from "./components/header";
import TokenListComponent from "./components/tokens";
import CoinsComponent from "./components/coins";
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
  return (
    <>
      <Header />
      <MessageComponent />
      <MarketHeader />
      <Contents>
        <CoinsComponent />
        <TokenListComponent />
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

import React, { FC } from "react";
import styled from "styled-components";
import ReactGA from "react-ga";

import { User } from "models/user";
import { MinPcWidth } from "app/components/responsive";
import Header from "app/components/header";
import { withUser, LoginStatus } from "app/components/user";
import NotFoundPage from "app/pages/not_found";

import MarketDataProvider, {
  useMarket
} from "./market/components/data_provider";
import MarketHeader from "./market/components/header";
import ParticipateComponent from "./market/components/participate";
import TokenListComponent from "./market/components/tokens";
import OrganizerComponent from "./market/components/organizer";
import CoinsComponent from "./market/components/coins";
import PrizeComponent from "./market/components/prize";
import DescComponent from "./market/components/description";

interface MarketPageProps {
  user: LoginStatus;
  marketId: string;
}

const MarketPage: FC<MarketPageProps> = ({ user, marketId }) => {
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

export default withUser(MarketPage);

const MarketPageInner: FC = () => {
  const { myHistory } = useMarket();
  return (
    <>
      <Header />
      <MarketHeader />
      <Contents>
        {!myHistory ? (
          <>
            <ParticipateComponent />
            <HR />
          </>
        ) : null}
        <TokenListComponent />
        <HR />
        <OrganizerComponent />
        <HR />
        <CoinsComponent />
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

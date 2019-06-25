import React, {FC, useState, useEffect} from 'react';
import {Redirect} from 'react-router-dom';
import {History} from 'history';

import User from 'models/user';
import {Market, MarketStatus} from 'models/market';
import {getMyMarkets} from 'api/user';
import {getMarkets} from 'api/market';
import {Pc, Tablet, Mobile} from 'components/responsive';
import PcContents from './account/pc';
import MobileContents from './account/mobile';

interface AccountPageProps {
  history: History;
  user: User | null;
}

const AccountPage: FC<AccountPageProps> = ({history, user}) => {
  const [participatedMarkets, setParticipatedMarkets] = useState<Market[]>([]);
  const [featuredMarkets, setFeaturedMarkets] = useState<Market[]>([]);

  useEffect(() => {
    if (user !== null) {
      getMyMarkets(user.accessToken).then(ms => setParticipatedMarkets(ms));
      getMarkets([MarketStatus.Upcoming, MarketStatus.Open]).then(res =>
        setFeaturedMarkets(res),
      );
    }
  }, [user]);

  if (user === null) {
    return <Redirect to={{pathname: '/login', state: {redirect: '/me'}}} />;
  } else {
    const mobile = (
      <MobileContents
        history={history}
        user={user}
        participatedMarkets={participatedMarkets}
        featuredMarkets={featuredMarkets}
      />
    );

    const pc = (
      <PcContents
        history={history}
        user={user}
        participatedMarkets={participatedMarkets}
        featuredMarkets={featuredMarkets}
      />
    );

    return (
      <>
        <Pc>{pc}</Pc>
        <Tablet>{mobile}</Tablet>
        <Mobile>{mobile}</Mobile>
      </>
    );
  }
};

export default AccountPage;

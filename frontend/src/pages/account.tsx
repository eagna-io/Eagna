import React, {FC, useState, useEffect} from 'react';
import {Redirect} from 'react-router-dom';
import {History} from 'history';

import User from 'models/user';
import {Market, MarketStatus} from 'models/market';
import {getMyMarkets} from 'api/user';
import {getMarkets} from 'api/market';
import Responsive from 'components/responsive';
import Pc from './account/pc';
import Mobile from './account/mobile';

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
    return (
      <Responsive
        renderPc={() => (
          <Pc
            history={history}
            user={user}
            participatedMarkets={participatedMarkets}
            featuredMarkets={featuredMarkets}
          />
        )}
        renderTablet={() => (
          <Mobile
            history={history}
            user={user}
            participatedMarkets={participatedMarkets}
            featuredMarkets={featuredMarkets}
          />
        )}
        renderMobile={() => (
          <Mobile
            history={history}
            user={user}
            participatedMarkets={participatedMarkets}
            featuredMarkets={featuredMarkets}
          />
        )}
      />
    );
  }
};

export default AccountPage;

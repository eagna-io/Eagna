import React, {FC} from 'react';
import styled from 'styled-components';
import {Link, RouteComponentProps, withRouter} from 'react-router-dom';

import {Market, OpenMarket} from 'models/market';
import {createInitialSupplyOrder} from 'api/market';
import {withUser, UserProps} from 'app/components/user';

interface Props {
  market: Market;
}

const ParticipateComponent: FC<Props & RouteComponentProps & UserProps> = ({
  market,
  user,
  location,
}) => {
  if (market instanceof OpenMarket) {
    if (user === 'Checking') {
      return (
        <Container>
          <CheckingDialog>Checking...</CheckingDialog>
          <Message>取引を行うにはログインする必要があります。</Message>
        </Container>
      );
    } else if (user === null) {
      return (
        <Container>
          <LoginButton
            to={{
              pathname: '/login',
              state: {redirect: location.pathname},
            }}>
            ログイン
          </LoginButton>
          <Message>取引を行うにはログインする必要があります。</Message>
        </Container>
      );
    } else {
      const requestParticipate = () => {
        user.getAccessToken().then(accessToken => {
          if (accessToken === null) {
            alert('すでにログアウトしています');
          } else {
            createInitialSupplyOrder(market.id, accessToken).then(res =>
              window.location.reload(),
            );
          }
        });
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
    }
  } else {
    return (
      <Container>
        <InactiveParticipateButton>
          参加する
        </InactiveParticipateButton>
        <Message>
          マーケットがOpen状態になると、「参加する」ボタンが押せるようになります。
        </Message>
      </Container>
    );
  }
};

export default withRouter(withUser(ParticipateComponent));

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
  background-color: #9B9B9B;
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
`;

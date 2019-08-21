import React, {FC} from 'react';
import styled from 'styled-components';

import {Market, Token} from 'models/market';
import {PriceHistory, MyAssets} from 'models/order';
import {LoginStatus, withUser} from 'app/components/user';

import ChartComponent from '../chart';
import AssetComponent from '../asset';
import OrderComponent from '../order';

interface Props {
  token: Token | null;
  market: Market;
  priceHistory: PriceHistory | null;
  myAssets: MyAssets | null;
  user: LoginStatus;
  onClose: () => void;
}

const TokenDetailPage: FC<Props> = React.memo(
  ({token, market, priceHistory, myAssets, user, onClose}) => {
    return (
      <Container show={token !== null}>
        {token ? (
          <TokenDetailPageContent
            token={token}
            market={market}
            priceHistory={priceHistory}
            myAssets={myAssets}
            user={user}
            onClose={onClose}
          />
        ) : null}
      </Container>
    );
  },
);

export default withUser(TokenDetailPage);

interface ContentProps {
  token: Token;
  market: Market;
  priceHistory: PriceHistory | null;
  myAssets: MyAssets | null;
  user: LoginStatus;
  onClose: () => void;
}

const TokenDetailPageContent: FC<ContentProps> = ({
  token,
  market,
  priceHistory,
  myAssets,
  user,
  onClose,
}) => {
  return (
    <>
      <CloseButton onClick={onClose} />
      <TokenName>{token.name}</TokenName>
      <Sumbnail src={token.sumbnailUrl} />
      <Description>{token.desc}</Description>
      <ChartComponent token={token} priceHistory={priceHistory} />
      <AssetComponent
        amountToken={myAssets ? myAssets.getTokenUncheck(token.name) : null}
        amountCoin={myAssets ? myAssets.getCoin() : null}
      />
      <OrderComponent
        token={token}
        market={market}
        myAssets={myAssets}
      />
    </>
  );
};

const Container = styled('div')<{show: boolean}>`
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 10;
  padding: 20px;
  background-color: white;
  overflow-x: hidden;
  overflow-y: scroll;
  -webkit-overflow-scrolling: touch;
  transform: ${props => (props.show ? 'scaleY(1)' : 'scaleY(0)')};
  transition: all 200ms 0s ease-out;
`;

const CloseButton = styled.div`
  position: absolute;
  right: 22px;
  top: 22px;
  width: 30px;
  height: 30px;
  border-radius: 15px;
  background-color: #f2f2f2;

  &:before,
  &:after {
    position: absolute;
    left: 14px;
    top: 4px;
    content: ' ';
    height: 22px;
    width: 3px;
    background-color: white;
  }
  &:before {
    transform: rotate(45deg);
  }
  &:after {
    transform: rotate(-45deg);
  }
`;

const TokenName = styled.h2`
  margin: 0;
  padding: 0;
  margin-top: 50px;
`;

const Sumbnail = styled('div')<{src: string}>`
  width: 100%;
  height: 135px;
  margin-top: 15px;
  background-image: url(${props => props.src});
  background-size: cover;
  background-position: center;
`;

const Description = styled.p`
  margin: 0;
  padding: 0;
  width: 100%;
  margin-top: 35px;
  color: #979797;
  font-size: 14px;
  line-height: 21px;
`;

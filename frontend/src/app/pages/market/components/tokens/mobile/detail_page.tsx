import React, {FC} from 'react';
import styled from 'styled-components';
import Chart from 'react-apexcharts';
import moment from 'moment';

import {
  Market,
  UpcomingMarket,
  OpenMarket,
  ClosedMarket,
  ResolvedMarket,
  Token,
} from 'models/market';
import {User} from 'models/user';
import {PriceHistory, MyAssets, NormalOrder} from 'models/order';
import {createNormalOrder} from 'api/market';
import {LoginStatus, withUser} from 'app/components/user';

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
  const chartData: [Date, number][] = priceHistory
    ? priceHistory.getHistoryOf(token.name)
    : [];
  const series = [
    {
      name: token.name,
      data: chartData,
    },
  ];

  return (
    <>
      <CloseButton onClick={onClose} />
      <TokenName>{token.name}</TokenName>
      <Sumbnail src={token.sumbnailUrl} />
      <Description>{token.desc}</Description>
      <ChartContainer>
        <Chart options={options} series={series} type="area" />
      </ChartContainer>
      <AssetContainer>
        <AssetLabel>所持枚数</AssetLabel>
        <AssetVolume>
          <AssetIcon src="/img/market/tokens.svg" />
          {myAssets ? myAssets.getTokenUncheck(token.name) : '-'}&nbsp;
          <AssetVolumeUnit>枚</AssetVolumeUnit>
        </AssetVolume>
      </AssetContainer>
      <AssetContainer>
        <AssetLabel>所持コイン</AssetLabel>
        <AssetVolume>
          <AssetIcon src="/img/market/coins.svg" />
          {myAssets ? myAssets.getCoin() : '-'}&nbsp;
          <AssetVolumeUnit>coin</AssetVolumeUnit>
        </AssetVolume>
      </AssetContainer>
      <OrderComponent
        token={token}
        market={market}
        myAssets={myAssets}
        user={user}
      />
    </>
  );
};

interface OrderComponentProps {
  token: Token;
  market: Market;
  myAssets: MyAssets | null;
  user: LoginStatus;
}

const OrderComponent: FC<OrderComponentProps> = ({
  token,
  market,
  myAssets,
  user,
}) => {
  let msg: string | null = null;
  let buyPrice: number | null = null;
  let buyable = false;
  let sellPrice: number | null = null;
  let sellable = false;
  let requestBuy: () => void = () => {};
  let requestSell: () => void = () => {};

  if (market instanceof UpcomingMarket) {
    msg = 'マーケットがOpen状態になると、取引が行えるようになります';
  } else if (market instanceof OpenMarket) {
    const buyAmountCoin = market.computeAmountCoinOfOrder(token.name, 1);
    const sellAmountCoin = market.computeAmountCoinOfOrder(token.name, -1);
    buyPrice = -buyAmountCoin;
    sellPrice = sellAmountCoin;

    if (myAssets === null) {
      msg = '「参加する」ボタンを押すと取引ができるようになります';
    } else {
      buyable = myAssets.getCoin() >= buyPrice;
      sellable = myAssets.getTokenUncheck(token.name) >= 1;
    }

    if (user instanceof User) {
      const requestOrder = (amountToken: number, amountCoin: number) => {
        user.getAccessToken().then(accessToken => {
          if (accessToken === null) {
            alert('もう一度ログインをお願いいたします。');
          } else {
            createNormalOrder(
              market.id,
              accessToken,
              new NormalOrder(token.name, amountToken, amountCoin, moment()),
            ).then(res => {
              console.log(res);
              window.location.reload();
            });
          }
        });
      };
      requestBuy = () => requestOrder(1, buyAmountCoin);
      requestSell = () => requestOrder(-1, sellAmountCoin);
    }
  } else if (
    market instanceof ClosedMarket ||
    market instanceof ResolvedMarket
  ) {
    msg = 'マーケットはすでにCloseしています';
  }

  return (
    <>
      {msg !== null ? <OrderNote>{msg}</OrderNote> : null}
      <OrderContainer>
        <OrderButtonContainer>
          <BuyButton disabled={!buyable} onClick={requestBuy}>
            {buyPrice === null ? '-' : buyPrice}
            <OrderButtonTextUnit>coin</OrderButtonTextUnit>
          </BuyButton>
          <OrderButtonDesc>で購入する</OrderButtonDesc>
        </OrderButtonContainer>
        <OrderButtonContainer>
          <SellButton disabled={!sellable} onClick={requestSell}>
            {sellPrice === null ? '-' : sellPrice}
            <OrderButtonTextUnit>coin</OrderButtonTextUnit>
          </SellButton>
          <OrderButtonDesc>で売却する</OrderButtonDesc>
        </OrderButtonContainer>
      </OrderContainer>
    </>
  );
};

const options = {
  chart: {
    stacked: false,
    zoom: {
      type: 'x',
      enabled: true,
    },
    toolbar: {
      show: false,
    },
  },
  plotOptions: {
    line: {
      curve: 'smooth',
    },
  },
  dataLabels: {
    enabled: false,
  },
  markers: {
    size: 0,
    style: 'full',
  },
  title: {
    show: false,
  },
  fill: {
    type: 'gradient',
    gradient: {
      shadeIntensity: 1,
      inverseColors: false,
      opacityFrom: 0.5,
      opacityTo: 0,
      stops: [0, 90, 100],
    },
  },
  grid: {
    show: false,
  },
  yaxis: {
    show: false,
  },
  xaxis: {
    type: 'datetime',
  },
  tooltip: {
    shared: false,
  },
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

const ChartContainer = styled.div`
  width: 100%;
  height: 250px;
  margin-top: 30px;
  margin-bottom: 35px;
`;

const AssetContainer = styled.div`
  display: flex;
  width: 100%;
  justify-content: space-between;
  padding: 0px 2% 35px 2%;
`;

const AssetLabel = styled.div`
  font-size: 16px;
  font-weight: bold;
`;

const AssetIcon = styled.img`
  width: 33px;
  height: 33px;
  margin-top: 5px;
  margin-right: 16px;
  vertical-align: top;
`;

const AssetVolume = styled.div`
  font-size: 30px;
  font-weight: bold;
`;

const AssetVolumeUnit = styled.span`
  font-size: 14px;
  font-weight: normal;
`;

const OrderNote = styled.p`
  width: 100%;
  font-size: 12px;
  color: #ff6666;
  padding: 0;
  margin: 0;
  text-align: center;
`;

const OrderContainer = styled.div`
  display: flex;
  width: 100%;
  justify-content: space-between;
  padding: 10px 2% 25px 2%;
`;

const OrderButtonContainer = styled.div`
  width: 130px;
`;

const OrderButton = styled.button`
  display: block;
  width: 100%;
  height: 50px;
  box-shadow: 1px 1px 4px 0 rgba(0, 0, 0, 0.5);
  border-radius: 4px;
  border: none;
  font-size: 18px;
  font-weight: bold;
  color: white;
  text-align: center;

  &:disabled {
    background-color: #9b9b9b;
  }
`;

const BuyButton = styled(OrderButton)`
  background-color: #358ed7;
`;

const SellButton = styled(OrderButton)`
  background-color: #d75035;
`;

const OrderButtonTextUnit = styled.span`
  font-size: 14px;
  font-weight: normal;
  margin-left: 5px;
`;

const OrderButtonDesc = styled.div`
  width: 100%;
  font-size: 12px;
  color: #979797;
  text-align: right;
  margin-top: 5px;
`;

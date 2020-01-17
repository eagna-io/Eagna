import React, { FC } from "react";
import styled from "styled-components";
import { useSelector, useDispatch } from "react-redux";

import { User } from "models/user";
import { MarketStatus, MarketToken } from "models/market";
import { Order, OrderRepository } from "models/order";
import { RootState } from "app/redux";
import { updateUserCoin } from "app/redux/user";
import { pc } from "app/components/responsive";

import { useMarket } from "../data_provider";

interface OrderComponentProps {
  token: MarketToken;
}

export default ({ token }: OrderComponentProps) => {
  const user = useSelector((state: RootState) => state.user.user);
  const { market, distribution, lmsr, myHistory, updateMarket } = useMarket();
  const [requesting, setRequesting] = React.useState(false);
  const dispatch = useDispatch();

  const currentCost = lmsr.computeCost();

  const buyPrice =
    distribution
      .add(token.name, 1)
      .lmsr(lmsr.B)
      .computeCost() - currentCost;

  let sellPrice = undefined;
  if (distribution.get(token.name) > 0) {
    sellPrice =
      currentCost -
      distribution
        .add(token.name, -1)
        .lmsr(lmsr.B)
        .computeCost();
  }
  console.log(sellPrice);

  if (market.status === MarketStatus.Upcoming) {
    return (
      <BaseOrderComponent
        token={token}
        msg="マーケットがOpen状態になると、取引が行えるようになります"
        buyPrice={buyPrice}
      />
    );
  } else if (
    market.status === MarketStatus.Closed ||
    market.status === MarketStatus.Resolved
  ) {
    return (
      <BaseOrderComponent
        token={token}
        msg="マーケットはすでにCloseしています"
        buyPrice={buyPrice}
        sellPrice={sellPrice}
      />
    );
  } else {
    if (!(user instanceof User)) {
      return (
        <BaseOrderComponent
          token={token}
          msg="ログインが必要です"
          buyPrice={buyPrice}
          sellPrice={sellPrice}
        />
      );
    } else {
      const buyable = user.coin >= buyPrice;
      const sellable = myHistory.assets.getToken(token.name) >= 1;
      const requestOrder = async (amountToken: number, amountCoin: number) => {
        setRequesting(true);
        const order = Order.normal({
          tokenName: token.name,
          amountToken,
          amountCoin
        });
        const res = await OrderRepository.create(market, user, order);
        console.log(res);
        await updateMarket();
        dispatch(updateUserCoin(user.coin + res.amountCoin));
        setRequesting(false);
      };
      return (
        <BaseOrderComponent
          token={token}
          buyable={buyable}
          sellable={sellable}
          buyPrice={buyPrice}
          sellPrice={sellPrice}
          requestOrder={requestOrder}
          requesting={requesting}
        />
      );
    }
  }
};

interface BaseOrderComponentProps {
  token: MarketToken;
  msg?: string;
  buyable?: boolean;
  sellable?: boolean;
  buyPrice: number;
  sellPrice?: number;
  requestOrder?: (amountToken: number, amountCoin: number) => void;
  requesting?: boolean;
}

const BaseOrderComponent: FC<BaseOrderComponentProps> = ({
  token,
  msg,
  buyable = false,
  sellable = false,
  buyPrice,
  sellPrice,
  requestOrder = () => {},
  requesting
}) => {
  return (
    <Container>
      {msg ? <OrderNote>{msg}</OrderNote> : null}
      <OrderButtonsContainer>
        <OrderButtonContainer>
          <BuyButton
            disabled={!buyable || requesting}
            onClick={() => requestOrder(1, -buyPrice)}
          >
            {buyPrice}
            <OrderButtonTextUnit>coin</OrderButtonTextUnit>
          </BuyButton>
          <OrderButtonDesc>で購入する</OrderButtonDesc>
          {buyable ? (
            <AdviceMsg>
              「{token.name}」が実現する可能性が
              <Percentage>{buyPrice / 10} %</Percentage>
              以上だと思うなら購入しましょう！
            </AdviceMsg>
          ) : null}
        </OrderButtonContainer>
        <OrderButtonContainer>
          <SellButton
            disabled={!sellable || requesting}
            onClick={() => requestOrder(-1, sellPrice || 0)}
          >
            {sellPrice || "-"}
            <OrderButtonTextUnit>coin</OrderButtonTextUnit>
          </SellButton>
          <OrderButtonDesc>で売却する</OrderButtonDesc>
          {sellable && sellPrice ? (
            <AdviceMsg>
              「{token.name}」が実現する可能性が
              <Percentage>{sellPrice / 10} %</Percentage>
              以下だと思うなら売却しましょう！
            </AdviceMsg>
          ) : null}
        </OrderButtonContainer>
      </OrderButtonsContainer>
    </Container>
  );
};

const Container = styled.div`
  width: 100%;
  margin-top: 30px;
`;

const OrderNote = styled.p`
  width: 100%;
  font-size: 12px;
  color: #ff6666;
  padding: 0;
  margin: 0;
  margin-bottom: 15px;
  text-align: center;

  ${pc(`
    margin-top: 50px;
  `)};
`;

const OrderButtonsContainer = styled.div`
  display: flex;
  width: 100%;
  justify-content: space-around;
  padding: 0px 2% 25px 2%;
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

const AdviceMsg = styled.p`
  margin-top: 10px;
  width: 100%;
  font-size: 9px;
  font-weight: normal;
`;

const Percentage = styled.span`
  margin: 0 5px;
  font-size: 10px;
  font-weight: bold;
`;

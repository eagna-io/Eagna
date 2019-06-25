import React, {FC, useState, useCallback} from 'react';
import styled from 'styled-components';
import {
  Token,
  TokenId,
  TokenDistribution,
  MyAssets,
  cloneTokenDistribution,
  distributionCost,
} from 'models/market';
import * as QuantityInputComponent from './order/elements/quantity_input';
import * as TokenSelectComponent from './order/elements/token_select';

enum OrderType {
  Buy = 'Buy',
  Sell = 'Sell',
}

interface OrderComponentProps {
  tokens: Token[];
  lmsrB: number;
  tokenDistribution: TokenDistribution;
  myAssets: MyAssets;
  requestOrder(token: Token, amountToken: number, amountCoin: number): void;
  className?: string;
}

export const Pc: FC<OrderComponentProps> = ({
  tokens,
  lmsrB,
  tokenDistribution,
  myAssets,
  requestOrder,
  className,
}) => {
  const [selectedToken, setSelectedToken] = useState<Token>(tokens[0]);
  const [orderType, setOrderType] = useState<OrderType>(OrderType.Buy);
  const [inputAmountToken, setAmountToken] = useState<number | null>(null);
  const [errMsg, setErr] = useState<string | null>(null);

  let futureDistribution = cloneTokenDistribution(tokenDistribution);
  if (inputAmountToken) {
    const curAmount = futureDistribution.get(selectedToken.id) || 0;
    const futAmount =
      orderType === OrderType.Buy
        ? curAmount + inputAmountToken
        : curAmount - inputAmountToken;
    futureDistribution.set(selectedToken.id, futAmount);
  }

  const cost =
    distributionCost(lmsrB, futureDistribution) -
    distributionCost(lmsrB, tokenDistribution);

  const onOrderButtonClick = useCallback(
    e => {
      e.preventDefault();
      if (cost === 0) {
        setErr('コストが0の注文は出せません。Quantityを大きくしてください。');
        return;
      }
      if (!inputAmountToken || inputAmountToken === 0) {
        setErr('トークンの量を入力してください');
        return;
      }
      if (orderType === OrderType.Buy) {
        if (!checkBalance(myAssets, 'Coin', cost)) {
          setErr('Coin の残高が不足しています');
          return;
        }
      }
      if (orderType === OrderType.Sell) {
        if (!checkBalance(myAssets, selectedToken.id, inputAmountToken)) {
          setErr('Token の残高が不足しています');
          return;
        }
      }
      setErr(null);
      // "buy" なら token は増える。"sell" なら逆
      const amountToken =
        orderType === OrderType.Buy ? inputAmountToken : -inputAmountToken;
      const amountCoin = -cost; // Coin の増量は cost の逆
      requestOrder(selectedToken, amountToken, amountCoin);
    },
    [
      cost,
      inputAmountToken,
      orderType,
      myAssets,
      selectedToken,
      setErr,
      requestOrder,
    ],
  );

  return (
    <PcContainer className={className}>
      <TokenSelectComponent.Pc tokens={tokens} onChange={setSelectedToken} />
      <OrderTypeSwitchComponent selected={orderType} onChange={setOrderType} />
      <PriceContainer>
        <QuantityInputComponent.Pc onChange={setAmountToken} />
        <PcPrice>
          {Math.abs(cost)}
          <PriceUnit>coins</PriceUnit>
        </PcPrice>
      </PriceContainer>
      {errMsg ? <h5>{errMsg}</h5> : null}
      <Separator />
      <OrderButton onClick={onOrderButtonClick}>Order</OrderButton>
    </PcContainer>
  );
};

export const Mobile: FC<OrderComponentProps> = ({
  tokens,
  lmsrB,
  tokenDistribution,
  myAssets,
  requestOrder,
  className,
}) => {
  const [selectedToken, setSelectedToken] = useState<Token>(tokens[0]);
  const [orderType, setOrderType] = useState<OrderType>(OrderType.Buy);
  const [inputAmountToken, setAmountToken] = useState<number | null>(null);
  const [errMsg, setErr] = useState<string | null>(null);

  let futureDistribution = cloneTokenDistribution(tokenDistribution);
  if (inputAmountToken) {
    const curAmount = futureDistribution.get(selectedToken.id) || 0;
    const futAmount =
      orderType === OrderType.Buy
        ? curAmount + inputAmountToken
        : curAmount - inputAmountToken;
    futureDistribution.set(selectedToken.id, futAmount);
  }

  const cost =
    distributionCost(lmsrB, futureDistribution) -
    distributionCost(lmsrB, tokenDistribution);

  const onPressEnter = () => {
    if (cost === 0) {
      setErr('コストが0の注文は出せません。Quantityを大きくしてください。');
      return;
    }
    if (!inputAmountToken || inputAmountToken === 0) {
      setErr('トークンの量を入力してください');
      return;
    }
    if (orderType === OrderType.Buy) {
      if (!checkBalance(myAssets, 'Coin', cost)) {
        setErr('Coin の残高が不足しています');
        return;
      }
    }
    if (orderType === OrderType.Sell) {
      if (!checkBalance(myAssets, selectedToken.id, inputAmountToken)) {
        setErr('Token の残高が不足しています');
        return;
      }
    }
    setErr(null);
    // "buy" なら token は増える。"sell" なら逆
    const amountToken =
      orderType === OrderType.Buy ? inputAmountToken : -inputAmountToken;
    const amountCoin = -cost; // Coin の増量は cost の逆
    requestOrder(selectedToken, amountToken, amountCoin);
  };

  return (
    <MobileContainer className={className}>
      <TokenSelectComponent.Mobile
        tokens={tokens}
        onChange={setSelectedToken}
      />
      <OrderTypeSwitchComponent selected={orderType} onChange={setOrderType} />
      <PriceContainer>
        <QuantityInputComponent.Mobile
          onChange={amount => setAmountToken(amount)}
        />
        <MobilePrice>
          {Math.abs(cost)}
          <PriceUnit>coins</PriceUnit>
        </MobilePrice>
      </PriceContainer>
      {errMsg ? <h5>{errMsg}</h5> : null}
      <Separator />
      <OrderButton
        onClick={e => {
          e.preventDefault();
          onPressEnter();
        }}>
        Order
      </OrderButton>
    </MobileContainer>
  );
};
const PcContainer = styled.div`
  width: 530px;
  height: 335px;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  padding: 34px;
  margin-top: 50px;
`;

const PriceContainer = styled.div`
  margin-top: 27px;
`;

const PcPrice = styled.div`
  display: inline-block;
  width: 190px;
  height: 40px;
  font-size: 32px;
  color: #979797;
  text-align: right;
  line-height: 40px;
  vertical-align: top;
`;

const MobileContainer = styled.div`
  width: 100%;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  padding: 24px;
  margin-top: 50px;
`;

const MobilePrice = styled.div`
  display: inline-block;
  width: 40%;
  height: 40px;
  font-size: 26px;
  color: #979797;
  text-align: right;
  line-height: 40px;
  vertical-align: top;
`;

const PriceUnit = styled.span`
  font-size: 14px;
  margin-left: 10px;
`;

const Separator = styled.hr`
  border: 0;
  border-top: 2px solid #4a90e2;
  margin-top: 33px;
  margin-bottom: 0px;
`;

const OrderButton = styled.button`
  width: 100%;
  height: 38px;
  border: 0;
  border-radius: 4px;
  background-color: #00c05e;
  color: white;
  font-size: 17px;
  margin-top: 20px;
`;

function checkBalance(
  myAssets: MyAssets,
  key: 'Coin' | TokenId,
  target: number,
): boolean {
  const balance = myAssets.get(key);
  if (balance === undefined) {
    throw `Logic error : MyAssets does not contain asset ${key}`;
  } else {
    if (balance < target) {
      return false;
    } else {
      return true;
    }
  }
}

interface OrderTypeSwitchComponentProps {
  selected: OrderType;
  onChange(orderType: OrderType): void;
}

const OrderTypeSwitchComponent: FC<OrderTypeSwitchComponentProps> = ({
  selected,
  onChange,
}) => {
  const Switch = styled.div`
    width: 100%;
    border-radius: 4px;
    border: 1px solid #d1d5da;
    margin-top: 28px;
  `;

  const BaseButton = styled.button`
    width: 50%;
    height: 35px;
    background-color: #f8f4f4;
    font-size: 14px;
    color: #37474f;
    border: none;
  `;

  const SelectedButton = styled(BaseButton)`
    color: white;
    background-color: #358ed7;
  `;

  if (selected === OrderType.Buy) {
    return (
      <Switch>
        <SelectedButton>Buy</SelectedButton>
        <BaseButton onClick={() => onChange(OrderType.Sell)}>Sell</BaseButton>
      </Switch>
    );
  } else {
    return (
      <Switch>
        <BaseButton onClick={() => onChange(OrderType.Buy)}>Buy</BaseButton>
        <SelectedButton>Sell</SelectedButton>
      </Switch>
    );
  }
};

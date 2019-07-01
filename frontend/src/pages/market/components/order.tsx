import React, {FC, useState} from 'react';
import {
  Token,
  TokenId,
  TokenDistribution,
  MyAssets,
  cloneTokenDistribution,
  distributionCost,
} from 'models/market';
import {OrderType} from './order/elements/order_type_switch';
import InternalPcComponent from './order/pc';
import InternalMobileComponent from './order/mobile';

interface OrderComponentProps {
  tokens: Token[];
  lmsrB: number;
  tokenDistribution: TokenDistribution;
  myAssets: MyAssets;
  requestOrder(token: Token, amountToken: number, amountCoin: number): void;
}

export interface InternalComponentProps {
  tokens: Token[];
  selectedToken: Token;
  onSelectedTokenChange: (token: Token) => void;
  orderType: OrderType;
  onOrderTypeChange: (type: OrderType) => void;
  amountToken: number | null;
  onAmountTokenChange: (n: number | null) => void;
  cost: number;
  errMsg: string | null;
  onOrderButtonClick: () => void;
}

function genComponent(
  InternalComponent: FC<InternalComponentProps>,
): FC<OrderComponentProps> {
  return ({tokens, lmsrB, tokenDistribution, myAssets, requestOrder}) => {
    const [selectedToken, setSelectedToken] = useState<Token>(tokens[0]);
    const [orderType, setOrderType] = useState<OrderType>(OrderType.Buy);
    const [amountToken, setAmountToken] = useState<number | null>(null);
    const [errMsg, setErr] = useState<string | null>(null);

    const cost =
      amountToken === null
        ? 0
        : calcCost(
            lmsrB,
            tokenDistribution,
            orderType,
            selectedToken.id,
            amountToken,
          );

    const onOrderButtonClick = () => {
      if (cost === 0) {
        setErr('0 Coin の注文は出せません。Quantityを大きくしてください。');
        return;
      }
      if (amountToken === null || amountToken === 0) {
        setErr('トークンの量を入力してください');
        return;
      }
      if (orderType === OrderType.Buy) {
        if (getBalance(myAssets, 'Coin') < cost) {
          setErr('Coin の残高が不足しています');
          return;
        }
      }
      if (orderType === OrderType.Sell) {
        if (getBalance(myAssets, selectedToken.id) < amountToken) {
          setErr('Token の残高が不足しています');
          return;
        }
      }

      setErr(null);
      // "buy" なら token は増える。"sell" なら逆
      const orderAmountToken =
        orderType === OrderType.Buy ? amountToken : -amountToken;
      const amountCoin = -cost; // Coin の増量は cost の逆
      requestOrder(selectedToken, orderAmountToken, amountCoin);
    };

    return (
      <InternalComponent
        tokens={tokens}
        selectedToken={selectedToken}
        onSelectedTokenChange={setSelectedToken}
        orderType={orderType}
        onOrderTypeChange={setOrderType}
        amountToken={amountToken}
        onAmountTokenChange={setAmountToken}
        cost={cost}
        errMsg={errMsg}
        onOrderButtonClick={onOrderButtonClick}
      />
    );
  };
}

export const Pc = genComponent(InternalPcComponent);
export const Mobile = genComponent(InternalMobileComponent);

function calcCost(
  lmsrB: number,
  tokenDistribution: TokenDistribution,
  orderType: OrderType,
  tokenId: TokenId,
  amountToken: number,
): number {
  const nextDistribution = cloneTokenDistribution(tokenDistribution);
  const curAmount = nextDistribution.get(tokenId) || 0;
  const nextAmount =
    orderType === OrderType.Buy
      ? curAmount + amountToken
      : curAmount - amountToken;
  nextDistribution.set(tokenId, nextAmount);

  return (
    distributionCost(lmsrB, nextDistribution) -
    distributionCost(lmsrB, tokenDistribution)
  );
}

function getBalance(myAssets: MyAssets, asset: 'Coin' | TokenId): number {
  const balance = myAssets.get(asset);
  if (balance === undefined) {
    throw `Logic error : MyAssets does not contain asset ${asset}`;
  } else {
    return balance;
  }
}

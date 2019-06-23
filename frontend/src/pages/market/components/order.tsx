import React, {FC, useState} from 'react';
import styled from 'styled-components';
import {
  Token,
  TokenId,
  TokenDistribution,
  MyAssets,
  cloneTokenDistribution,
  distributionCost,
} from 'models/market';

const MAX_QUANTITY = 1000;

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

const OrderComponent: FC<OrderComponentProps> = ({
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
    <Container className={className}>
      <TokenSelectComponent
        selected={selectedToken}
        tokens={tokens}
        onChange={setSelectedToken}
      />
      <OrderTypeSwitchComponent selected={orderType} onChange={setOrderType} />
      <PriceContainer>
        <QuantityInput
          type="text"
          value={inputAmountToken || ''}
          placeholder="Quantity"
          onChange={e => {
            setAmountToken(validateAmountToken(e.target.value));
          }}
        />
        <Price>
          {Math.abs(cost)}
          <PriceUnit>coins</PriceUnit>
        </Price>
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
    </Container>
  );
};

export default OrderComponent;

function validateAmountToken(input: string): number | null {
  if (input === '') {
    return null;
  }
  const n = parseInt(input);
  if (Number.isNaN(n)) {
    alert('Not a number');
    return null;
  } else {
    return Math.min(Math.max(n, 0), MAX_QUANTITY);
  }
}

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

const Container = styled.div`
  width: 530px;
  height: 335px;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  padding: 34px;
`;

const PriceContainer = styled.div`
  margin-top: 27px;
`;

const QuantityInput = styled.input`
  width: 270px;
  height: 40px;
  border-radius: 4px;
  border: 1px solid #d1d5da;
  font-size: 14px;
  color: #979797;
  padding-left: 20px;
`;

const Price = styled.div`
  display: inline-block;
  width: 190px;
  height: 40px;
  font-size: 32px;
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

interface TokenSelectComponentProps {
  selected: Token;
  tokens: Token[];
  onChange(token: Token): void;
}

const TokenSelectComponent: FC<TokenSelectComponentProps> = ({
  selected,
  tokens,
  onChange,
}) => {
  return (
    <Select
      name="token"
      value={selected.name}
      onChange={e => {
        const token = tokens.find(t => t.name === e.target.value);
        if (token) {
          onChange(token);
        }
      }}>
      {tokens.map(token => (
        <option value={token.name} key={token.name}>
          {token.name}
        </option>
      ))}
    </Select>
  );
};

const Select = styled.select`
  width: 100%;
  height: 40px;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  background-color: white;
  padding: 0 20px;
  font-family: Lucida Grande;
  font-size: 14px;
  color: #37474f;
`;

interface OrderTypeSwitchComponentProps {
  selected: OrderType;
  onChange(orderType: OrderType): void;
}

const OrderTypeSwitchComponent: FC<OrderTypeSwitchComponentProps> = ({
  selected,
  onChange,
}) => {
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

import React, {useContext, useState} from 'react';
import styled from 'styled-components';
import * as lmsr from 'src/lmsr';
import {AccessTokenContext} from 'src/context';
import {postOrder, getMarket,
  InvalidAccessTokenError, TokenPriceIsMovedError
} from 'src/api';

const MAX_QUANTITY = 100;
const MICRO_COIN = 1000000;

export default function  Order(props) {
  const {tokens, marketId, setErr, setMarket } = props;
  const accessToken = useContext(AccessTokenContext).token;
  const [selectedToken, setSelectedToken] = useState(null);
  const [orderType, setOrderType] = useState("buy");
  const [amountToken, setAmountToken] = useState(null);

  const cost = selectedToken !== null ? currentCost(selectedToken, amountToken, tokens, orderType) : 0;

  const requestOrder = () => {
    postOrder(
      selectedToken.id,
      orderType === "buy" ? amountToken : -amountToken, // buy なら tokenは増える
      -cost, // Coin の増量は cost の逆
      accessToken
    )
      .catch(err => {
        switch(err) {
          case InvalidAccessTokenError:
            setToken(null);
            setErr(["You need to login", Date.now()]);
            break;
          case TokenPriceIsMovedError:
            setErr(["Price of the token is changed", Date.now()]);
            break;
          default:
            setErr(["Invalid order", Date.now()]);
            break;
        }
      })
      // Error が検知された場合も market 情報を取得し直す
      .then(() => getMarket(marketId, accessToken))
      .then(market => setMarket(market))
  };

  const checkSelectedToken = () => {
    if (selectedToken === null) {
      setErr(["Please select the token", Date.now()]);
      return false;
    }
    return true
  };

  const checkAmountToken = () => {
    if (amountToken === null) {
      setErr(["Please input amount of the token", Date.now()]);
      return false;
    }
    return true
  }

  return (
    <Container className={props.className}>
      <TokenSelect
        selected={selectedToken}
        tokens={props.tokens}
        onChange={e => {
          const token = tokens.find(t => t.name === e.target.value);
          setSelectedToken(token);
        }} />
      <OrderTypeSwitch
        selected={orderType}
        onChange={type => setOrderType(type)} />
      <PriceContainer>
        <QuantityInput
          type="text"
          value={amountToken || ""}
          placeholder="Quantity"
          onChange={e => {
            setAmountToken(validateAmountToken(e.target.value));
          }}/>
        <Price>
          { Math.abs(cost) }
          <PriceUnit>coins</PriceUnit>
        </Price>
      </PriceContainer>
      <Separator />
      <OrderButton onClick={(e) => {
        e.preventDefault();
        checkSelectedToken() &&
        checkAmountToken() &&
        requestOrder();
      }}>
        Order
      </OrderButton>
    </Container>
  );
}

function validateAmountToken(input) {
  if (input === "") {
    return null;
  }
  const n = parseInt(input);
  if (!Number.isNaN(n) && n < MAX_QUANTITY){
    return n;
  } else {
    return null;
  }
}

function currentCost(token, amountToken, tokens, orderType) {
  const baseCost = lmsr.cost(tokens.map(t => t.amount));
  const newCost = lmsr.cost(tokens.map(t => {
    if (t.id === token.id) {
      if (orderType === "buy") {
        return t.amount + amountToken;
      } else {
        return t.amount - amountToken;
      }
    } else {
      return t.amount 
    }
  }));
  return newCost - baseCost;
}


const Container = styled.div`
  width: 530px;
  border: 1px solid #D1D5DA;
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
  border: 1px solid #D1D5DA;
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
  border-top: 2px solid #4A90E2;
  margin-top: 33px;
  margin-bottom: 0px;
`;

const OrderButton = styled.button`
  width: 100%;
  height: 38px;
  border: 0;
  border-radius: 4px;
  background-color: #00C05E;
  color: white;
  font-size: 17px;
  margin-top: 20px;
`;

function TokenSelect(props) {
  const value = props.selected === null ? "" : props.selected.name;
  return (
    <Select name="token" value={value} onChange={props.onChange}>
      {
        props.selected === null
        ? (<Placeholder value="" disabled>Select Token</Placeholder>)
        : null
      }
      {
        props.tokens.map(token => (
          <option value={token.name} key={token.name}>
            { token.name }
          </option>
        ))
      }
    </Select>
  );
}

const Select = styled.select`
  width: 100%;
  height: 40px;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
  background-color: white;
  padding: 0 20px;
  font-family: Lucida Grande;
  font-size: 14px;
  color: #37474F;
`;

const Placeholder = styled.option`
  display: none;
`;


function OrderTypeSwitch(props) {
  if (props.selected == "buy") {
    return (
      <Switch>
        <SelectedButton>Buy</SelectedButton>
        <BaseButton onClick={() => props.onChange("sell")}>Sell</BaseButton>
      </Switch>
    );
  } else {
    return (
      <Switch>
        <BaseButton onClick={() => props.onChange("buy")}>Buy</BaseButton>
        <SelectedButton>Sell</SelectedButton>
      </Switch>
    );
  }
}

const Switch = styled.div`
  width: 100%;
  border-radius: 4px;
  border: 1px solid #D1D5DA;
  margin-top: 28px;
`;

const BaseButton = styled.button`
  width: 50%;
  height: 35px;
  background-color: #F8F4F4;
  font-size: 14px;
  color: #37474F;
  border: none;
`;

const SelectedButton = styled(BaseButton)`
  color: white;
  background-color: #358ED7;
`;

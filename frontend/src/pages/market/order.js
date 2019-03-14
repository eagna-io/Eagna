import React from 'react';
import styled from 'styled-components';
import * as lmsr from 'src/lmsr';

const MAX_QUANTITY = 100;
const MICRO_COIN = 1000000;

export default class  Order extends React.Component {
  constructor(props) {
    super(props);
    this.onTokenChange = this.onTokenChange.bind(this);
    this.onTypeChange = this.onTypeChange.bind(this);
    this.onQuantityChange = this.onQuantityChange.bind(this);
    this.onSubmit = this.onSubmit.bind(this);
    this.currentCost = this.currentCost.bind(this);
    this.state = {
      token: null,
      type: "buy",
      quantity: null,
    };
  }

  onTokenChange(e) {
    this.setState({
      token: this.props.tokens.find(t => t.name == e.target.value),
    });
  }

  onTypeChange(orderType) {
    this.setState({
      type: orderType,
    });
  }

  onQuantityChange(e) {
    if (e.target.value == "") {
      this.setState({
        quantity: null,
      });
      return;
    }

    const n = parseInt(e.target.value);
    if (!Number.isNaN(n) && n < MAX_QUANTITY) {
      this.setState({
        quantity: n,
      });
    }
  }

  onSubmit(e) {
    if (this.state.token === null || this.state.quantity === null) {
      return;
    }
    this.props.requestOrder(
      this.state.token.id,
      this.state.type === "buy" ? this.state.quantity : -this.state.quantity,
      -this.currentCost(), // 100 のコストであれば、持ちコインが100 減る
      this.props.marketId,
      this.props.accessToken,
    )
  }

  currentCost() {
    if (this.state.token === null || this.state.quantity === null) {
      return 0;
    }

    const baseCost = lmsr.cost(this.props.tokens.map(t => t.amount));
    const newCost = lmsr.cost(this.props.tokens.map(t => {
      if (t.id === this.state.token.id) {
        if (this.state.type === "buy") {
          return t.amount + this.state.quantity;
        } else {
          return t.amount - this.state.quantity;
        }
      } else {
        return t.amount 
      }
    }));
    return newCost - baseCost;
  }

  render() {
    
    return (
      <Container className={this.props.className}>
        <TokenSelect
          selected={this.state.token}
          tokens={this.props.tokens}
          onChange={this.onTokenChange}/>
        <OrderTypeSwitch
          selected={this.state.type}
          onChange={this.onTypeChange}/>
        <PriceContainer>
          <QuantityInput
            type="text"
            value={this.state.quantity || ""}
            placeholder="Quantity"
            onChange={this.onQuantityChange}/>
          <Price>
            { Math.abs(this.currentCost()) }
            <PriceUnit>coins</PriceUnit>
          </Price>
        </PriceContainer>
        <Separator />
        <OrderButton onClick={this.onSubmit}>Order</OrderButton>
      </Container>
    );
  }
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

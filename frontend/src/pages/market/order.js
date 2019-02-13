import React from 'react';
import styled from 'styled-components';

export default class  Order extends React.Component {
  constructor(props) {
    super(props);
    this.onTokenChange = this.onTokenChange.bind(this);
    this.onTypeChange = this.onTypeChange.bind(this);
    this.onQuantityChange = this.onQuantityChange.bind(this);
    this.state = {
      token: {
        name: "",
        price: "-",
      },
      type: "buy",
      quantity: "",
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
        quantity: "",
      });
      return;
    }

    const n = parseInt(e.target.value);
    if (!Number.isNaN(n)) {
      this.setState({
        quantity: n,
      });
    }
  }

  render() {
    const price = this.state.token.price == "-" ? "0.0" : this.state.token.price * this.state.quantity;
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
            value={this.state.quantity}
            placeholder="Quantity"
            onChange={this.onQuantityChange}/>
          <Price>
            { price }
            <PriceUnit>coins</PriceUnit>
          </Price>
        </PriceContainer>
        <Separator />
        <OrderButton>Order</OrderButton>
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
  return (
    <Select name="token" value={props.selected.name} onChange={props.onChange}>
      {
        props.selected.name == ""
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

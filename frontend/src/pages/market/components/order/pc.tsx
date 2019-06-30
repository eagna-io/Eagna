import React, {FC} from 'react';
import styled from 'styled-components';
import {Pc as QuantityInputElement} from './elements/quantity_input';
import {Pc as TokenSelectElement} from './elements/token_select';
import {Pc as OrderTypeSwitchElement} from './elements/order_type_switch';
import {InternalComponentProps} from '../order';

const Component: FC<InternalComponentProps> = ({
  tokens,
  selectedToken,
  onSelectedTokenChange,
  orderType,
  onOrderTypeChange,
  amountToken,
  onAmountTokenChange,
  cost,
  errMsg,
  onOrderButtonClick,
}) => {
  return (
    <Container>
      <TokenSelectElement
        tokens={tokens}
        selected={selectedToken}
        onChange={onSelectedTokenChange}
      />
      <OrderTypeSwitchElement
        selected={orderType}
        onChange={onOrderTypeChange}
      />
      <PriceContainer>
        <QuantityInputElement
          value={amountToken}
          onChange={onAmountTokenChange}
        />
        <Price>
          {Math.abs(cost)}
          <PriceUnit>coins</PriceUnit>
        </Price>
      </PriceContainer>
      {errMsg ? <h5>{errMsg}</h5> : null}
      <Separator />
      <OrderButton onClick={onOrderButtonClick}>Order</OrderButton>
    </Container>
  );
};

export default Component;

const Container = styled.div`
  width: 530px;
  border: 1px solit #d1d5da;
  border-radius: 4px;
  padding: 34px;
  margin-top: 50px;
`;

const PriceContainer = styled.div`
  margin-top: 27px;
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
  margin-toop: 33px;
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

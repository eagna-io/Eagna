import React, {FC} from 'react';
import styled from 'styled-components';

export enum OrderType {
  Buy = 'Buy',
  Sell = 'Sell',
}

interface Props {
  selected: OrderType;
  onChange(orderType: OrderType): void;
}

export const Pc: FC<Props> = ({selected, onChange}) => {
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

export const Mobile = Pc;

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

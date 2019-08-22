import React from 'react';
import styled from 'styled-components';

import {pc} from 'app/components/responsive';

interface Props {
  amountToken: number | null;
  amountCoin: number | null;
}

const AssetComponent: React.FC<Props> = ({amountToken, amountCoin}) => {
  return (
    <Container>
      <AssetContainer>
        <Label>所持枚数</Label>
        <Amount>
          <Icon src="/img/market/tokens.svg" />
          {amountToken === null ? '-' : amountToken}&nbsp;
          <AmountUnit>枚</AmountUnit>
        </Amount>
      </AssetContainer>
      <AssetContainer>
        <Label>所持コイン</Label>
        <Amount>
          <Icon src="/img/market/coins.svg" />
          {amountCoin === null ? '-' : amountCoin}&nbsp;
          <AmountUnit>coin</AmountUnit>
        </Amount>
      </AssetContainer>
    </Container>
  );
};

export default AssetComponent;

const Container = styled.div`
  width: 100%;
  margin-top: 5px;
  padding: 0px 2%;
`;

const AssetContainer = styled.div`
  display: flex;
  width: 100%;
  justify-content: space-between;
  margin-top: 30px;

  ${pc(`
    display: inline-block;
    width: 150px;
    
    &:first-of-type {
      margin-right: calc(100% - 300px);
    }
  `)}
`;

const Label = styled.div`
  font-size: 16px;
  font-weight: bold;
`;

const Amount = styled.div`
  font-size: 30px;
  font-weight: bold;

  ${pc(`
    width: 100%;
    margin-top: 25px;
    text-align: right;
  `)}
`;

const AmountUnit = styled.span`
  font-size: 14px;
  font-weight: bold;
`;

const Icon = styled.img`
  width: 33px;
  height: 33px;
  margin-top: 5px;
  margin-right: 16px;
  vertical-align: top;
`;

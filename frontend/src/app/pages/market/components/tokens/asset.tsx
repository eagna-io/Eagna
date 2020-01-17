import React from "react";
import styled from "styled-components";
import { useSelector } from "react-redux";

import { MarketToken } from "models/market";
import { pc } from "app/components/responsive";
import { RootState } from "app/redux";

import { useMarket } from "../data_provider";

interface Props {
  token: MarketToken;
}

const AssetComponent: React.FC<Props> = ({ token }) => {
  const { myHistory } = useMarket();
  const myAssets = myHistory ? myHistory.assets : undefined;
  const user = useSelector((state: RootState) => state.user.user);

  return (
    <Container>
      <AssetContainer>
        <Label>所持枚数</Label>
        <Amount>
          <Icon src="/img/market/tokens.svg" />
          {myAssets ? myAssets.getToken(token.name) : "-"}&nbsp;
          <AmountUnit>枚</AmountUnit>
        </Amount>
      </AssetContainer>
      <AssetContainer>
        <Label>所持チップ</Label>
        <Amount>
          <Icon src="/img/market/coins.svg" />
          {user ? user.coin : "-"}&nbsp;
          <AmountUnit>枚</AmountUnit>
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
    width: 180px;
    
    &:first-of-type {
      margin-right: calc(100% - (180px * 2));
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
    margin-top: 10px;
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

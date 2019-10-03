import React, { FC } from "react";
import styled from "styled-components";

import { MarketToken } from "models/market";

import ChartComponent from "../chart";
import AssetComponent from "../asset";
import OrderComponent from "../order";

interface Props {
  token: MarketToken | null;
  onClose: () => void;
}

export default React.memo(({ token, onClose }: Props) => {
  return (
    <Container show={token !== null}>
      {token ? (
        <TokenDetailPageContent token={token} onClose={onClose} />
      ) : null}
    </Container>
  );
});

interface ContentProps {
  token: MarketToken;
  onClose: () => void;
}

const TokenDetailPageContent: FC<ContentProps> = ({ token, onClose }) => {
  return (
    <>
      <CloseButton onClick={onClose} />
      <TokenName>{token.name}</TokenName>
      <Sumbnail src={token.thumbnailUrl} />
      <Description>{token.description}</Description>
      <ChartComponent token={token} />
      <AssetComponent token={token} />
      <OrderComponent token={token} />
    </>
  );
};

const Container = styled("div")<{ show: boolean }>`
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 10;
  padding: 20px;
  background-color: white;
  overflow-x: hidden;
  overflow-y: scroll;
  -webkit-overflow-scrolling: touch;
  transform: ${props => (props.show ? "scaleY(1)" : "scaleY(0)")};
  transition: all 200ms 0s ease-out;
`;

const CloseButton = styled.div`
  position: absolute;
  right: 22px;
  top: 22px;
  width: 30px;
  height: 30px;
  border-radius: 15px;
  background-color: #f2f2f2;

  &:before,
  &:after {
    position: absolute;
    left: 14px;
    top: 4px;
    content: " ";
    height: 22px;
    width: 3px;
    background-color: white;
  }
  &:before {
    transform: rotate(45deg);
  }
  &:after {
    transform: rotate(-45deg);
  }
`;

const TokenName = styled.h2`
  margin: 0;
  padding: 0;
  margin-top: 50px;
`;

const Sumbnail = styled("div")<{ src: string }>`
  width: 100%;
  height: 135px;
  margin-top: 15px;
  background-image: url(${props => props.src});
  background-size: cover;
  background-position: center;
`;

const Description = styled.p`
  margin: 0;
  padding: 0;
  width: 100%;
  margin-top: 35px;
  color: #979797;
  font-size: 14px;
  line-height: 21px;
`;

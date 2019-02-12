import React from 'react';
import styled from 'styled-components';

export default function Tokens(props) {
  return (
    <Container className={props.className}>
      <thead>
        <Header>
          <HeaderToken>Token</HeaderToken>
          <HeaderPrice>Price</HeaderPrice>
          <HeaderDesc>Description</HeaderDesc>
        </Header>
      </thead>
      <tbody>
        { props.tokens.map((token, idx) => (
          <Item filled={idx % 2 == 1} key={token.name}>
            <ItemToken>{token.name}</ItemToken>
            <ItemPrice>{token.price}</ItemPrice>
            <ItemDesc>{token.desc}</ItemDesc>
          </Item>
          ))
        }
      </tbody>
    </Container>
  );
}

const Container = styled.table`
  width: 100%;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
  border-spacing: 0;
  border-collapse: collapse;
`;

const Header = styled.tr`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  font-weight: normal;
  background-color: #F6F8FA;
  height: 40px;
  border: none;
`;

const HeaderToken = styled.th`
  padding-left: 75px;
  text-align: left;
  width: 20%;
`;

const HeaderPrice = styled.th`
  width: 10%;
  text-align: right;
`;

const HeaderDesc = styled.th`
  width: 70%;
  padding-left: 100px;
  padding-right: 30px;
  text-align: left;
`;

const Item = styled.tr`
  height: 50px;
  border-top: 1px solid #D1D5DA;
  background-color: ${props => props.filled ? "#F9F9F9" : "white" };
`;

const ItemToken = styled.td`
  color: #37474F;
  font-size: 14px;
  font-family: Lucida Grande;
  font-weight: normal;
  padding-left: 75px;
`;

const ItemPrice = styled.td`
  color: #37474F;
  font-size: 16px;
  font-family: Lucida Grande;
  font-weight: normal;
  text-align: right;
`;

const ItemDesc = styled.td`
  padding-left: 100px;
  padding-right: 30px;
  color: #979797;
  font-size: 14px;
  font-family: Lucida Grande;
  font-weight: normal;
  text-align: left;
`;

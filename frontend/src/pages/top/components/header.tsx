import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';

export const Pc: FC<{}> = () => {
  const Header = styled.header`
    width: 100%;
    height: 140px;
  `;

  const Logo = styled.img`
    position: absolute;
    width: 200px;
    height: 100px;
    left: 50px;
    top: 20px;
  `;

  const HeaderLinks = styled.div`
    position: absolute;
    display: flex;
    height: 100px;
    right: 0px;
    top: 20px;
    flex-direction: row-reverse;
  `;

  const HeaderLink = styled.a`
    height: 100px;
    line-height: 100px;
    margin-right: 70px;
    font-weight: 300;
    font-size: 20px;
    color: #1b384e;
    letter-spacing: 0;
  `;

  return (
    <Header>
      <Logo src="/img/logo.png" />
      <HeaderLinks>
        <HeaderLink>操作説明</HeaderLink>
        <HeaderLink>Eagnaとは？</HeaderLink>
        <HeaderLink>予測市場とは？</HeaderLink>
      </HeaderLinks>
    </Header>
  );
};

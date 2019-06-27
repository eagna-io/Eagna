import React, {FC} from 'react';
import styled from 'styled-components';
import {Link} from 'react-router-dom';

export const Pc: FC<{marketId: number}> = ({marketId}) => {
  return (
    <ContainerPc>
      <DescPc>オーダーを出すためにはログインが必要です。</DescPc>
      <SigninPc
        to={{pathname: '/login', state: {redirect: `/market/${marketId}`}}}>
        ログイン
      </SigninPc>
    </ContainerPc>
  );
};

const ContainerPc = styled.div`
  width: 400px;
  border: 1px solid lightgray;
  border-radius: 4px;
  margin-top: 50px;
`;

const DescPc = styled.div`
  width: 100%;
  padding: 50px;
  color: #979797;
  font-size: 16px;
`;

const SigninPc = styled(Link)`
  display: block;
  width: 150px;
  height: 30px;
  margin: 0 auto;
  margin-bottom: 50px;
  background-color: #1C384C
  border-radius: 4px;
  color: white;
  text-align: center;
  
  &:visited {
    color: white;
  }
`;

export const Mobile: FC<{marketId: number}> = ({marketId}) => {
  return (
    <ContainerMobile>
      <DescMobile>オーダーを出すためにはログインが必要です。</DescMobile>
      <SigninMobile
        to={{pathname: '/login', state: {redirect: `/market/${marketId}`}}}>
        ログイン
      </SigninMobile>
    </ContainerMobile>
  );
};

const ContainerMobile = styled.div`
  width: 100%;
  border: 1px solid lightgray;
  border-radius: 4px;
  margin-top: 50px;
`;

const DescMobile = styled.div`
  width: 100%;
  padding: 50px;
  color: #979797;
  font-size: 16px;
`;

const SigninMobile = styled(Link)`
  display: block;
  width: 150px;
  height: 30px;
  margin: 0 auto;
  margin-bottom: 50px;
  background-color: #1C384C
  border-radius: 4px;
  color: white;
  text-align: center;
  
  &:visited {
    color: white;
  }
`;

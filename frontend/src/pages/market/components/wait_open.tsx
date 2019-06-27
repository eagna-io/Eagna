import React, {FC} from 'react';
import styled from 'styled-components';

export const Pc: FC = () => {
  return (
    <ContainerPc>
      <DescPc>マーケットがオープンすると、注文を出すことができます</DescPc>
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

export const Mobile: FC = () => {
  return (
    <ContainerMobile>
      <DescMobile>
        マーケットがオープンすると、注文を出すことができます
      </DescMobile>
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

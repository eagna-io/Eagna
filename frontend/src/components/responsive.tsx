import React, {FC} from 'react';
import styled from 'styled-components';

// 980px 以上
export const Pc: FC<{}> = ({children}) => {
  const Container = styled.div`
    @media (max-width: 979px) {
      display: none;
    }
  `;

  return <Container>{children}</Container>;
};

// 768px以上 980px 未満
export const Tablet: FC<{}> = ({children}) => {
  const Container = styled.div`
    @media (max-width: 767px) {
      display: none;
    }
    @media (min-width: 980px) {
      display: none;
    }
  `;

  return <Container>{children}</Container>;
};

// 768px未満
export const Mobile: FC<{}> = ({children}) => {
  const Container = styled.div`
    @media (min-width: 767px) {
      display: none;
    }
  `;

  return <Container>{children}</Container>;
};

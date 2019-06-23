import React, {FC} from 'react';
import styled from 'styled-components';

import {Token} from 'models/market';

interface ResultComponentProps {
  settleToken?: Token;
  className?: string;
}

const ResultComponent: FC<ResultComponentProps> = ({
  settleToken,
  className,
}) => {
  const result = settleToken
    ? `結果 [ ${settleToken.name} ]`
    : '結果の発表まで今しばらくおまちください.';
  return (
    <Container className={className}>
      <Header>Result</Header>
      <Content>{result}</Content>
    </Container>
  );
};

export default ResultComponent;

const Container = styled.div`
  width: 530px;
  border: 1px solid #d1d5da;
  border-radius: 4px;
`;

const Header = styled.div`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  font-weight: bold;
  background-color: #f6f8fa;
  height: 40px;
  padding-left: 40px;
  line-height: 40px;
  border-bottom: 1px solid #d1d5da;
`;

const Content = styled.div`
  color: #358ed7;
  font-size: 17px;
  font-family: Lucida Grande;
  font-weight: normal;
  line-height: 1.5;
  text-align: center;
  padding: 40px;
`;

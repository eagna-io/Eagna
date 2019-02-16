import React from 'react';
import styled from 'styled-components';

export default function Description(props) {
  const result = props.result || "The result will be determined soon.";
  return (
    <Container className={props.className}>
      <Header>Result</Header>
      <Content>{result}</Content>
    </Container>
  );
}

const Container = styled.div`
  width: 530px;
  border: 1px solid #D1D5DA;
  border-radius: 4px;
`;

const Header = styled.div`
  color: #586069;
  font-size: 12px;
  font-family: Lucida Grande;
  font-weight: bold;
  background-color: #F6F8FA;
  height: 40px;
  padding-left: 40px;
  line-height: 40px;
  border-bottom: 1px solid #D1D5DA;
`;

const Content = styled.div`
  color: #358ED7;
  font-size: 17px;
  font-family: Lucida Grande;
  font-weight: normal;
  line-height: 1.5;
  text-align: center;
  padding: 40px;
`;

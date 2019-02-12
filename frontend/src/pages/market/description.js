import React from 'react';
import styled from 'styled-components';

export default function Description(props) {
  return (
    <Container className={props.className}>
      <Header>Description</Header>
      <Content>{props.content}</Content>
    </Container>
  );
}

const Container = styled.div`
  width: 100%;
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
  color: #24292E;
  font-size: 16px;
  font-family: Lucida Grande;
  font-weight: normal;
  line-height: 1.5;
  padding: 40px;
`;

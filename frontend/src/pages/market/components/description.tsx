import React, {FC} from 'react';
import styled from 'styled-components';
import ReactMarkdown from 'react-markdown';

interface DescriptionComponentProps {
  content: string;
  className?: string;
}

export const Pc: FC<DescriptionComponentProps> = ({
  content,
  className,
}) => {
  const Container = styled.div`
    width: 100%;
    border: 1px solid #d1d5da;
    border-radius: 4px;
    margin-top: 50px;
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
    color: #24292e;
    font-size: 16px;
    font-family: Lucida Grande;
    font-weight: normal;
    line-height: 1.5;
    padding: 40px;
  `;

  return (
    <Container className={className}>
      <Header>Description</Header>
      <Content>
        <ReactMarkdown source={content} />
      </Content>
    </Container>
  );
};

export const Mobile: FC<DescriptionComponentProps> = ({
  content,
  className,
}) => {
  const Container = styled.div`
    width: 100%;
    border: 1px solid #d1d5da;
    border-radius: 4px;
    margin-top: 50px;
  `;

  const Header = styled.div`
    color: #586069;
    font-size: 12px;
    font-weight: bold;
    background-color: #f6f8fa;
    height: 40px;
    padding-left: 20px;
    line-height: 40px;
    border-bottom: 1px solid #d1d5da;
  `;

  const Content = styled.div`
    color: #24292e;
    font-size: 14px;
    font-family: Lucida Grande;
    font-weight: normal;
    line-height: 1.5;
    padding: 20px;

    h1 {
      font-size: 22px;
      margin: 10px 0;
    }

    h2 {
      font-size: 18px;
      margin: 10px 0;
    }

    h3 {
      font-size: 16px;
      margin: 10px 0;
    }

    li {
      font-size: 14px;
    }
  `;

  return (
    <Container className={className}>
      <Header>Description</Header>
      <Content>
        <ReactMarkdown source={content} />
      </Content>
    </Container>
  );
};

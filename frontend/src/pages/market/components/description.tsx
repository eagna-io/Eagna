import React, {FC} from 'react';
import styled from 'styled-components';
import ReactMarkdown from 'react-markdown';

interface DescriptionComponentProps {
  content: string;
  className?: string;
}

const DescriptionComponent: FC<DescriptionComponentProps> = ({
  content,
  className,
}) => {
  return (
    <Container className={className}>
      <Header>Description</Header>
      <Content>
        <ReactMarkdown source={content} />
      </Content>
    </Container>
  );
};

export default DescriptionComponent;

const Container = styled.div`
  width: 100%;
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
  color: #24292e;
  font-size: 16px;
  font-family: Lucida Grande;
  font-weight: normal;
  line-height: 1.5;
  padding: 40px;
`;

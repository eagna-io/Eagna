import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import ReactMarkdown from 'react-markdown';

const PrivacyPolicy: FC = () => {
  const [text, setText] = useState('');

  useEffect(() => {
    fetch('/txt/privacy_policy.txt')
      .then(res => res.text())
      .then(txt => setText(txt));
  }, []);

  return (
    <Container>
      <ReactMarkdown>{text}</ReactMarkdown>
    </Container>
  );
};

export default PrivacyPolicy;

const Container = styled.div`
  width: 90%;
  max-width: 980px;
  margin: 0 auto;
  padding: 50px;
  font-size: 14px;
  line-height: 3;
`;

import React, {FC, useState, useEffect} from 'react';
import styled from 'styled-components';
import ReactMarkdown from 'react-markdown';

const PlainText: FC<{textUrl: string}> = ({textUrl}) => {
  const [text, setText] = useState('');

  useEffect(() => {
    fetch(textUrl)
      .then(res => res.text())
      .then(txt => setText(txt));
  }, [textUrl]);

  return (
    <Container>
      <ReactMarkdown>{text}</ReactMarkdown>
    </Container>
  );
};

export default PlainText;

const Container = styled.div`
  width: 90%;
  max-width: 980px;
  margin: 0 auto;
  padding: 50px;
  font-size: 14px;
  line-height: 3;
`;

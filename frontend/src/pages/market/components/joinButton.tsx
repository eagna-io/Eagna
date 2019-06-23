import React, {FC} from 'react';
import styled from 'styled-components';

interface JoinButtonComponentProps {
  requestJoin(): void;
  className?: string;
}

const JoinButtonComponent: FC<JoinButtonComponentProps> = ({
  requestJoin,
  className,
}) => {
  return (
    <Container className={className}>
      <Desc>
        下にある「参加する」ボタンを押すと、コインが配布され、オーダーが出せるようになります。
      </Desc>
      <JoinButton onClick={() => requestJoin()}>参加する</JoinButton>
    </Container>
  );
};

export default JoinButtonComponent;

const Container = styled.div`
  width: 400px;
  border: 1px solid lightgray;
  border-radius: 4px;
`;

const Desc = styled.div`
  width: 100%;
  padding: 50px;
  color: #979797;
  font-size: 16px;
`;

const JoinButton = styled.button`
  display: block;
  width: 150px;
  height: 30px;
  margin: 0 auto;
  margin-bottom: 50px;
  background-color: #1C384C
  border-radius: 4px;
  color: white;
`;

import React, {FC, useState, useCallback} from 'react';
import styled from 'styled-components';

const MAX_QUANTITY = 100;

interface Props {
  onChange(amount: number | null): void;
  className?: string;
}

const Base: FC<Props> = React.memo(({onChange, className}) => {
  const [inputValue, setInputValue] = useState<number | null>(null);

  const onInputChange = useCallback(
    event => {
      const amount = validateAmount(event.target.value);
      setInputValue(amount);
      onChange(amount);
    },
    [setInputValue, onChange],
  );

  return (
    <Container
      className={className}
      type="text"
      value={inputValue !== null ? inputValue : ''}
      placeholder="トークンの量を入力"
      onChange={onInputChange}
    />
  );
});

const Container = styled.input`
  width: 248px;
  height: 40px;
  border-radius: 4px;
  border: 1px solid #d1d5da;
  font-size: 14px;
  color: #979797;
  padding-left: 20px;
`;

export const Pc = styled(Base)`
  width: 248px;
  height: 40px;
  font-size: 14px;
  padding-left: 20px;
`;

export const Mobile = styled(Base)`
  width: 60%;
  height: 40px;
  font-size: 12px;
  padding-left: 10px;
`;

function validateAmount(input: string): number | null {
  if (input === '') {
    return null;
  }
  const n = parseInt(input);
  if (Number.isNaN(n)) {
    alert('数字を入力してください');
    return null;
  } else {
    return Math.min(Math.max(n, 0), MAX_QUANTITY);
  }
}

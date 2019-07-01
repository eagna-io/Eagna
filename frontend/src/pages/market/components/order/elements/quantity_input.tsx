import React, {FC, useCallback} from 'react';
import styled from 'styled-components';

const MAX_QUANTITY = 100;

interface Props {
  value: number | null;
  onChange(amount: number | null): void;
  className?: string;
}

const Base: FC<Props> = React.memo(({onChange, value, className}) => {
  const onInputChange = useCallback(
    event => {
      const amount = validateAmount(event.target.value);
      onChange(amount);
    },
    [onChange],
  );

  return (
    <Container
      type="text"
      value={value !== null ? value : ''}
      placeholder="トークンの量を入力"
      onChange={onInputChange}
      className={className}
    />
  );
});

const Container = styled.input`
  border-radius: 4px;
  border: 1px solid #d1d5da;
  color: #979797;
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

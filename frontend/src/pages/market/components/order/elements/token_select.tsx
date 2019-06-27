import React, {FC, useState, useCallback, useMemo} from 'react';
import styled from 'styled-components';
import {Token} from 'models/market';

interface Props {
  tokens: Token[];
  onChange(token: Token): void;
}

const Base: FC<Props> = React.memo(({tokens, onChange}) => {
  const [selected, setSelected] = useState<Token>(tokens[0]);

  const onChangeCallback = useCallback(
    e => {
      const token = tokens.find(t => t.name === e.target.value);
      if (token) {
        setSelected(token);
        onChange(token);
      }
    },
    [tokens, onChange, setSelected],
  );

  const selectOptions = useMemo(
    () =>
      tokens.map(token => (
        <option value={token.name} key={token.name}>
          {token.name}
        </option>
      )),
    [tokens],
  );

  return (
    <Select name="token" value={selected.name} onChange={onChangeCallback}>
      {selectOptions}
    </Select>
  );
});

const Select = styled.select`
  width: 100%;
  height: 40px;
  border: 1px solid #d1d5da;
  border-radius: 4px;
  background-color: white;
  padding: 0 20px;
  font-family: Lucida Grande;
  font-size: 14px;
  color: #37474f;
`;

export const Pc = styled(Base)``;
export const Mobile = styled(Base)``;

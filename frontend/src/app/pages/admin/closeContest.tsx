import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";

import { AdminTemplate } from "./components/template/admin";

export const CloseContest: React.FC = () => {
  return (
    <AdminTemplate>
      <Table>
        <Tr>
          <Th align="left">カテゴリー</Th>
          <Th align="left">タイトル</Th>
          <Th align="center">開始時刻</Th>
          <Th align="center">CLOSE</Th>
        </Tr>
        <Tr>
          <Td align="left">NBA（バスケ</Td>
          <Td align="left">Los Angels Lakers vs Golden State Warriors</Td>
          <Td align="center">開始時刻</Td>
          <Td align="center"><Submit>CLOSE</Submit></Td>
        </Tr>
      </Table>
    </AdminTemplate>
  );
};

const Table = styled.table`
  width: 720px;
  padding: 10px 8px;
  margin: 0 auto;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
  border-collapse:  collapse;
`;

const Tr = styled.tr`
`;

const Th = styled.th<{ align: string }>`
  padding: 8px 12px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  text-align: ${props => props.align};
`;

const Td = styled.td<{ align: string }>`
  padding: 8px 12px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  text-align: ${props => props.align};
`;

const Submit = styled.button`
  width: 50px;
  height: 20px;
  padding: 4px 8px;
  border-radius: 2px;
  background-color: ${color.MainRed.hex};
  color: ${color.WhiteBaseColor.hex};
  font-size: 10px;
  &:disabled {
    background-color: ${color.AdminInputBorderColor.hex};
  }
`;

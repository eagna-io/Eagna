import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import { Contest } from "model/contest";

import { AdminTemplate } from "./components/template/admin";

interface Props {
  contests: Contest[];
}

export const CloseContest: React.FC<Props> = ({ contests }) => {
  return (
    <AdminTemplate>
      <Table>
        <Tr>
          <Th align="left">カテゴリー</Th>
          <Th align="left">タイトル</Th>
          <Th align="center">開始時刻</Th>
          <Th align="center">CLOSE</Th>
        </Tr>
        {contests.map(contest => (
          <Tr>
            <Td align="left">{contest.category}</Td>
            <Td align="left">{contest.title}</Td>
            <Td align="center">{contest.startAt}</Td>
            <Td align="center">
              <Submit disabled={contest.status === "closed" || contest.status === "archived"}>CLOSE</Submit>
            </Td>
          </Tr>
        ))}
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

const Submit = styled.button<{ disabled: boolean }>`
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

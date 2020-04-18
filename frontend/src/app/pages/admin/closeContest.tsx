import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";

import { AdminTemplate } from "./components/template/admin";

export const CloseContest: React.FC = () => {
  return (
    <AdminTemplate>
      <TextInputWrapper>
        <Tag>カテゴリー</Tag>
        <Input type="text" placeholder="例）NBA（バスケ）" />
      </TextInputWrapper>
      <TextInputWrapper>
         <Tag>タイトル</Tag>
         <Input type="text" placeholder="例）Los Angels Lakers vs Golden State Warriors" />
      </TextInputWrapper>
      <TextInputWrapper>
         <Tag>開始時間</Tag>
         <Input
          type="datetime-local"
          placeholder="開始時刻を入力してください。例）2020/04/18 19:00"
        />
      </TextInputWrapper>
      <Submit>作成</Submit>
    </AdminTemplate>
  );
};

const TextInputWrapper = styled.div`
  display: flex;
  justify-content: space-between;
  margin: 64px 40px 0 0;
`;

const Tag = styled.div`
  font-size: 14px;
  font-weight: 500;
  line-height: 30px;
  color: ${color.AdminMainColor.hex};
`;

const Input = styled.input`
  width: 526px;
  height: 30px;
  padding: 10px 8px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
`;

const Submit = styled.button`
  display: block;
  position: absolute;
  bottom: 31px;
  left: 50%;
  transform: translateY(0%) translateX(-50%);
  -webkit-transform: translateY(0%) translateX(-50%);
  width: 250px;
  height: 40px;
  background-color: ${color.MainRed.hex};
  font-size: 14px;
  color: ${color.WhiteBaseColor.hex};
`;

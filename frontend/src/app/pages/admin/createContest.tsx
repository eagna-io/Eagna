import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import * as contestApi from "infra/http/contest";
import * as storage from "infra/storage";

import { AdminTemplate } from "./components/template/admin";

export const CreateContest: React.FC = () => {
  const [accessToken] = React.useState(storage.getAdminAccessToken);
  const [category, setCategory] = React.useState("");
  const [title, setTitle] = React.useState("");
  const [startAt, setStartAt] = React.useState("");

  return (
    <AdminTemplate>
      <TextInputWrapper>
        <Tag>カテゴリー</Tag>
        <Input
          type="text"
          placeholder="例）NBA（バスケ）"
          value={category}
          onChange={e => setCategory(e.target.value)}
        />
      </TextInputWrapper>
      <TextInputWrapper>
        <Tag>タイトル</Tag>
        <Input
          type="text"
          placeholder="例）Los Angels Lakers vs Golden State Warriors"
          value={title}
          onChange={e => setTitle(e.target.value)}
        />
      </TextInputWrapper>
      <TextInputWrapper>
        <Tag>開始時間</Tag>
        <Input
          type="text"
          placeholder="開始時刻を入力してください。例）2014-10-10T04:50:40Z"
          value={startAt}
          onChange={e => setStartAt(e.target.value)}
        />
      </TextInputWrapper>
      <Submit
        onClick={() => {
          if (!accessToken) {
            alert("ログインが必要です");
            return;
          }
          contestApi
            .post(accessToken, title, category, startAt)
            .then(res => alert("コンテストが作成されました"));
        }}
      >
        作成
      </Submit>
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

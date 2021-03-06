import React from "react";
import styled from "styled-components";

import * as color from "app/components/color";
import * as adminApi from "infra/http/admin";
import * as storage from "infra/storage";

import { AdminTemplate } from "./components/template/admin";

export const AdminLogin: React.FC = () => {
  const [email, setEmail] = React.useState("");
  const [pass, setPass] = React.useState("");

  return (
    <AdminTemplate>
      <IdWrapper>
        <Tag>メールアドレス</Tag>
        <MailAddress
          type="text"
          placeholder="email"
          value={email}
          onChange={e => setEmail(e.target.value)}
        />
      </IdWrapper>
      <PassWrapper>
        <Tag>パスワード</Tag>
        <Password
          type="password"
          placeholder="password"
          value={pass}
          onChange={e => setPass(e.target.value)}
        />
      </PassWrapper>
      <Submit
        onClick={() => {
          adminApi.post(email, pass).then(res => {
            storage.setAdminAccessToken(res.access_token);
            alert("ログインしました");
          });
        }}
      >
        ログイン
      </Submit>
    </AdminTemplate>
  );
};

const IdWrapper = styled.div`
  display: flex;
  justify-content: space-between;
  margin: 100px 40px 0 0;
`;

const Tag = styled.div`
  font-size: 14px;
  font-weight: 500;
  line-height: 30px;
  color: ${color.AdminMainColor.hex};
`;

const MailAddress = styled.input`
  width: 526px;
  height: 30px;
  padding: 10px 8px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
`;

const PassWrapper = styled.div`
  display: flex;
  justify-content: space-between;
  margin: 56px 40px 0 0;
`;

const Password = styled.input`
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
  background-color: ${color.Correct.hex};
  font-size: 14px;
  color: ${color.WhiteBaseColor.hex};
`;

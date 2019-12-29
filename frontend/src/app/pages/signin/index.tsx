import React, { FC, useState, useEffect } from "react";
import styled from "styled-components";
import { withRouter } from "react-router-dom";
import { History } from "history";
import ReactGA from "react-ga";
import { useSelector, useDispatch } from "react-redux";
import TextField from "@material-ui/core/TextField";

import { User } from "models/user";
import { EagnaUserApi } from "infra/eagna/user";
import { Storage } from "infra/storage";
import { RootState } from "app/redux";
import { setUser } from "app/redux/user";

interface Props {
  history: History;
}

const SigninPageWrapper: FC<Props> = ({ history }) => {
  const user = useSelector((state: RootState) => state.user.user);
  const [email, setEmail] = useState("");
  const [pass, setPass] = useState("");
  const dispatch = useDispatch();

  useEffect(() => {
    ReactGA.pageview("/account");
  }, []);

  useEffect(() => {
    if (user) {
      history.push("/account");
    }
  });

  const onClick = async () => {
    // アクセストークンを取得
    const token = await EagnaUserApi.createAccessToken({
      email,
      password: pass
    });
    if (!token) {
      // TODO : alertを使わない
      alert("ログインに失敗しました");
      return;
    }

    // アクセストークンを保存
    Storage.setToken(token);

    // ユーザー情報を取得
    const user = await EagnaUserApi.queryMe(token);
    if (!user) {
      alert("Something goes wrong...");
      return;
    }

    // Stateを更新
    dispatch(setUser(User.fromInfra(user, token)));

    // アカウントページへ遷移
    history.push("/account");
  };

  return (
    <>
      <HeaderLogo src="/img/logo.png" />
      <InputForm>
        <StyledTextField
          variant="outlined"
          margin="dense"
          label="メールアドレス"
          onChange={e => setEmail(e.target.value)}
        />
        <StyledTextField
          variant="outlined"
          margin="dense"
          label="パスワード"
          type="password"
          onChange={e => setPass(e.target.value)}
        />
        <SubmitButton onClick={onClick}>ログイン</SubmitButton>
      </InputForm>
    </>
  );
};

export default withRouter(SigninPageWrapper);

const HeaderLogo = styled.img`
  width: 114px;
  height: 40px;
  margin: 0 auto auto 0;
`;

const InputForm = styled.div`
  width: 90%;
  margin: 133px auto 0 auto;
  padding: 50px 0;
`;

const StyledTextField = styled(TextField)`
  width: 100%;
  margin-bottom: 21px;
`;

const SubmitButton = styled.button`
  display: block;
  margin: 35px auto 0 auto;
  width: 205px;
  height: 43px;
  border-radius: 3.4px;
  box-shadow: 1px 1px 3px 0 rgba(0, 0, 0, 0.5);
  background-color: #358ed7;
  font-family: NotoSansJP;
  font-size: 14px;
  font-weight: 500;
  line-height: 43px;
  text-align: center;
  color: white;
`;

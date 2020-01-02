import React, { useState, useEffect } from "react";
import styled from "styled-components";
import { useHistory, useParams } from "react-router-dom";
import ReactGA from "react-ga";
import { useDispatch } from "react-redux";
import TextField from "@material-ui/core/TextField";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import Checkbox from "@material-ui/core/Checkbox";
import jwt_decode from "jwt-decode";

import { User } from "models/user";
import { EagnaUserApi } from "infra/eagna/user";
import { Storage } from "infra/storage";
import { setUser } from "app/redux/user";
import { pc } from "app/components/responsive";

export default () => {
  const history = useHistory();
  const { token } = useParams();
  const [name, setName] = useState("");
  const [pass, setPass] = useState("");
  const [pass2, setPass2] = useState("");
  const [confirmed, setConfirmed] = useState(false);
  const dispatch = useDispatch();

  const email = decodeJWT(token || "");

  useEffect(() => {
    if (!email) {
      alert("会員登録は招待されたユーザーのみ行えます。");
    }
  }, [email]);

  useEffect(() => {
    ReactGA.pageview("/signup");
  }, []);

  const onClick = async () => {
    if (token === undefined) {
      return;
    }

    // 新規ユーザー作成
    const accessToken = await EagnaUserApi.create({
      name,
      password: pass,
      invitationToken: token
    });

    if (accessToken === null) {
      alert("このメールアドレスは既に登録済みです");
      return;
    }

    // アクセストークンを保存
    Storage.setToken(accessToken);

    // ユーザー情報を取得
    const user = await EagnaUserApi.queryMe(accessToken);
    if (!user) {
      alert("Something goes wrong...");
      return;
    }

    // Stateを更新
    dispatch(setUser(User.fromInfra(user, accessToken)));

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
          value={email}
          disabled={true}
          onChange={e => setName(e.target.value)}
        />
        <StyledTextField
          variant="outlined"
          margin="dense"
          label="ユーザーネーム"
          autoFocus={true}
          onChange={e => setName(e.target.value)}
        />
        <StyledTextField
          variant="outlined"
          margin="dense"
          label="パスワード"
          type="password"
          onChange={e => setPass(e.target.value)}
        />
        <StyledTextField
          variant="outlined"
          margin="dense"
          label="パスワード確認"
          type="password"
          onChange={e => setPass2(e.target.value)}
          error={pass2 !== "" && pass !== pass2}
          helperText={
            pass2 !== "" && pass !== pass2 ? "パスワードが一致しません。" : null
          }
        />
        <FormControlLabel
          control={
            <Checkbox
              onChange={e => setConfirmed(e.target.checked)}
              color="primary"
            />
          }
          label={
            <Terms>
              <a href="/terms">CROPサービス利用規約</a>、
              <a href="/privacy">プライバシーポリシー</a>、
              <a href="/terms-point">CROPサービスポイント規約</a>
              を読んだ上で内容に同意します。
            </Terms>
          }
        />
        <SubmitButton
          disabled={name === "" || pass === "" || pass !== pass2 || !confirmed}
          onClick={onClick}
        >
          利用規約に同意して登録
        </SubmitButton>
      </InputForm>
    </>
  );
};

const decodeJWT = (token: string): string => {
  try {
    const decoded = jwt_decode<JWTClaim>(token).email;
    console.warn(decoded);
    return decoded;
  } catch {
    return "";
  }
};

interface JWTClaim {
  email: string;
}

const HeaderLogo = styled.img`
  width: 228px;
  height: 80px;
  margin: 0 auto auto 0;
`;

const InputForm = styled.div`
  width: 90%;
  max-width: 622px;
  margin: 20px auto 0 auto;

  ${pc(`
    margin-top: 291px;
  `)}
`;

const StyledTextField = styled(TextField)`
  width: 100%;
  margin-bottom: 21px;
`;

const Terms = styled.p`
  font-size: 11px;
  font-weight: normal;
  text-align: justify;
  color: #1b384e;

  & a {
    font-weight: bold;
    color: #0000ee;
  }

  & a:visited {
    color: #0000ee;
  }
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

  &:disabled {
    background-color: #9b9b9b;
  }
`;

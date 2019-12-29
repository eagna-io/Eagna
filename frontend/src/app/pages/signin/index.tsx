import React, { FC, useState, useEffect } from "react";
import styled from "styled-components";
import { withRouter } from "react-router-dom";
import { History } from "history";
import ReactGA from "react-ga";
import { useSelector } from "react-redux";
import TextField from "@material-ui/core/TextField";

import { Market, MarketStatus, MarketRepository } from "models/market";
import { User } from "models/user";
import { EagnaUserApi } from "infra/eagna/user";
import { RootState } from "app/redux";
import { MinPcWidth } from "app/components/responsive";
import Header from "app/components/header";

interface Props {
  history: History;
}

const SigninPageWrapper: FC<Props> = ({ history }) => {
  const user = useSelector((state: RootState) => state.user.user);

  useEffect(() => {
    ReactGA.pageview("/account");
  }, []);

  useEffect(() => {
    if (user) {
      history.push("/account");
    }
  });

  return <SigninPage />;
};

export default withRouter(SigninPageWrapper);

const SigninPage: FC = () => {
  const [email, setEmail] = useState("");
  const [pass, setPass] = useState("");

  const onClick = async () => {
    const token = await EagnaUserApi.createAccessToken({
      email,
      password: pass
    });
    alert(token);
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

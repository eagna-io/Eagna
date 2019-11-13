import React from "react";
import { useDispatch } from "react-redux";
import styled from "styled-components";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import Typography from "@material-ui/core/Typography";
import TextField from "@material-ui/core/TextField";
import Button from "@material-ui/core/Button";
import { withRouter } from "react-router-dom";
import { History } from "history";

import { UserRepository } from "models/user";
import { setUser } from "app/redux/user";

interface Props {
  history: History;
}

const SigninForm: React.FC<Props> = ({ history }) => {
  const [email, setEmail] = React.useState();
  const [password, setPassword] = React.useState();

  const dispatch = useDispatch();

  const onClick = () => {
    if (!email || !password) {
      return;
    }
    (async () => {
      const user = await UserRepository.signin(email, password);
      if (!user) {
        alert("メールアドレスまたはパスワードが違います");
      } else {
        dispatch(setUser(user));
        history.push("/account");
      }
    })();
  };

  return (
    <Grid container justify="center">
      <Grid item xs={4} sm={3}>
        <Container>
          <Typography variant="h6" gutterBottom align="center">
            ログイン
          </Typography>
          <TextField
            label="メールアドレス"
            variant="outlined"
            placeholder="info@crop-pm.com"
            fullWidth
            margin="dense"
            error={email === ""}
            helperText={
              email === "" ? "メールアドレスを入力してください" : undefined
            }
            onChange={e => setEmail(e.target.value)}
          />
          <TextField
            label="パスワード"
            variant="outlined"
            type="password"
            fullWidth
            margin="dense"
            error={password === ""}
            helperText={
              password === "" ? "パスワードを入力してください" : undefined
            }
            onChange={e => setPassword(e.target.value)}
          />
          <LoginButton variant="contained" color="primary" onClick={onClick}>
            ログイン
          </LoginButton>
        </Container>
      </Grid>
    </Grid>
  );
};

export default withRouter(SigninForm);

const Container = styled(Paper)`
  padding: 30px 50px;
  background-color: white;
`;

const LoginButton = styled(Button)`
  display: block;
  margin: 15px auto 0 auto;
`;

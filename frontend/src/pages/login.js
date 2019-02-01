import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import { requestLogin } from '../actions';

const Container = styled.div`
  margin-top: 30vh;
  margin-left: 30vw;
  width: 40vw;
`;

const Title = styled.div`
  color: #84B6F9;
  margin-left: 30vw;
  width: 40vw;
`;

const Input = styled.input`
  width: 100%;
  height: 40px;
  margin-top: 20px;
  font-size: 20px;
  padding-left: 10px;
`;

const SubmitButton = styled.button`
  width: 100%;
  height: 50px;
  margin-top: 20px;
  background-color: #84B6F9;
  color: white;
  font-size: 20px;
  font-family: "Arial";
`;

/*
 * onLoginSuccess
 */
class LoginPage extends React.Component {
  constructor(props) {
    super(props);
    this.onChange = this.onChange.bind(this);
    this.onKeyPress = this.onKeyPress.bind(this);
    this.onPressLogin = this.onPressLogin.bind(this);
    this.state = {
      userName: "",
      password: "",
    };
  }

  onChange(event) {
    const target = event.target;
    this.setState({
      [target.name]: target.value
    })
  }

  onKeyPress(event) {
    if (event.which == 13) {
      event.preventDefault();
      const name = this.state.userName;
      const rawPass = this.state.password;
      this.props.requestLogin(name, rawPass);
    }
  }

  onPressLogin(event) {
    event.preventDefault(); 
    const name = this.state.userName;
    const rawPass = this.state.password;
    this.props.requestLogin(name, rawPass);
  }

  render() {
    if (this.props.accessToken != null) {
      return <Redirect to="/me" />
    }
    if (this.props.isRequesting) {
      return <h3>Requesting...</h3>
    }
    const failedMessage = this.props.showFailed ? <h3>username or password is incorrect</h3> : null
    return (
      <Container>
        <Title>ROHAN MARKET</Title>
        { failedMessage }
        <Input
          type="text"
          name="userName"
          placeholder="User Name"
          onChange={this.onChange} 
          onKeyPress={this.onKeyPress} />
        <Input
          type="text"
          name="password"
          placeholder="Password"
          onChange={this.onChange}
          onKeyPress={this.onKeyPress} />
        <SubmitButton
          onClick={this.onPressLogin}>
            Login
        </SubmitButton>
      </Container>
    );
  }
}

function mapStateToProps(state) {
  return {
    isRequesting: state.pages.login.isRequesting,
    showFailed: state.pages.login.showFailed,
    accessToken: state.me.accessToken,
  }
}

function mapDispatchToProps(dispatch) {
  return {
    requestLogin: (name, rawPass) => {
      dispatch(requestLogin(name, rawPass))
    }
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(LoginPage)

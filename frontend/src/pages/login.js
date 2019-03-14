import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import { requestLogin } from '../actions';

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
      email: "",
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
      const name = this.state.email;
      const rawPass = this.state.password;
      this.props.requestLogin(name, rawPass);
    }
  }

  onPressLogin(event) {
    event.preventDefault(); 
    const email = this.state.email;
    const rawPass = this.state.password;
    this.props.requestLogin(email, rawPass);
  }

  render() {
    if (this.props.accessToken) {
      return <Redirect to="/me" />
    }
    if (this.props.isRequesting) {
      return <h3>Requesting...</h3>
    }
    const failedMessage = this.props.showFailed ? <h3>email or password is incorrect</h3> : null
    return (
      <Body>
        <Container>
          <Title>ROHAN MARKET</Title>
          { failedMessage }
          <Input
            type="text"
            name="email"
            placeholder="email"
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
      </Body>
    );
  }
}

const Body = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: #F5F8FD;
  position: absolute;
  top: 0px;
  left: 0px;
`;

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


function mapStateToProps(state) {
  return {
    ...state.pages.login,
    accessToken: state.me.accessToken,
  }
}

function mapDispatchToProps(dispatch) {
  return {
    requestLogin: (email, rawPass) => {
      dispatch(requestLogin(email, rawPass))
    }
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(LoginPage)

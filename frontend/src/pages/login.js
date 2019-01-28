import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';

import { requestLogin } from '../actions';
import css from './login.css';

/*
 * onLoginSuccess
 */
class LoginPage extends React.Component {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
    this.onPressLogin = this.onPressLogin.bind(this);
    this.state = {
      userName: "",
      password: "",
    };
  }

  handleChange(event) {
    const target = event.target;
    this.setState({
      [target.name]: target.value
    })
  }

  onPressLogin(event) {
    event.preventDefault(); 
    const name = this.state.userName;
    const rawPass = this.state.password;
    this.props.requestLogin(name, rawPass);
  }

  render() {
    if (this.props.accessToken !== undefined) {
      return <Redirect to="/me" />
    }
    if (this.props.isRequesting) {
      return <h3>Requesting...</h3>
    }
    return (
      <div className={css.container}>
        <div className={css.title}>ROHAN MARKET</div>
        <input
          className={css.input}
          type="text"
          name="userName"
          placeholder="User Name"
          onChange={this.handleChange} />
        <input
          className={css.input}
          type="text"
          name="password"
          placeholder="Password"
          onChange={this.handleChange} />
        <button
          className={css.button}
          onClick={this.onPressLogin}>Login</button>
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    isRequesting: state.login.isRequesting,
    accessToken: state.login.accessToken,
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

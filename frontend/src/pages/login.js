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
          onChange={this.onChange} 
          onKeyPress={this.onKeyPress} />
        <input
          className={css.input}
          type="text"
          name="password"
          placeholder="Password"
          onChange={this.onChange}
          onKeyPress={this.onKeyPress} />
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

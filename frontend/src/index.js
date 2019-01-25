import React from 'react';
import ReactDOM from 'react-dom';
import LoginPage from './pages/login';
import AccountPage from './pages/account';
import css from './index.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.onLoginSuccess = this.onLoginSuccess.bind(this);
    this.state = {
      accessToken: props.accessToken,
    };
  }

  onLoginSuccess(accessToken) {
    console.log("AccessToek = " + accessToken)
    this.setState({
      accessToken: accessToken,
    });
    this.fetchUserInfo()
  }

  fetchUserInfo() {
    if (this.state.accessToken === undefined) {
      return;
    }
    fetch("http://localhost:8099/users?access_token="+this.state.accessToken)
      .then(res => res.json())
      .then(
        this.setUserInfo,
        err => {
          console.log("Failed to fetch user data")
          alert("something went wrong")
        }
      )
  }

  setUserInfo(res) {
    if (res.success == false) {
      console.log("Failed to fetch user data")
      alert("something went wrong")
      return;
    }
    const user = res.result.user
    this.setState({
      user: {
        name: user.name,
        holdCoin: user.holdCoin,
      },
    });
  }

  render() {
    if (this.state.accessToken === undefined) {
      return (
        <div id={css.app}>
          <LoginPage onLoginSuccess={this.onLoginSuccess}/>
        </div>
      );
    } else {
      return (
        <div id={css.app}>
          <AccountPage user={this.state.user}/>
        </div>
      );
    }
  }
}

ReactDOM.render(<App />, document.getElementById('app'))

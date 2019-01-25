import React from 'react';
import ReactDOM from 'react-dom';
import LoginPage from './pages/login';
import AccountPage from './pages/account';
import css from './index.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.onLoginSuccess = this.onLoginSuccess.bind(this);
    this.fetchMyInfo = this.fetchMyInfo.bind(this);
    this.setMyInfo = this.setMyInfo.bind(this);
    this.state = {
      accessToken: props.accessToken,
    };
  }

  onLoginSuccess(accessToken) {
    console.log("AccessToek = " + accessToken)
    this.setState({
      accessToken: accessToken,
    });
    this.fetchMyInfo()
  }

  fetchMyInfo() {
    if (this.state.accessToken === undefined) {
      return;
    }
    fetch("http://localhost:8099/me?access_token="+this.state.accessToken)
      .then(res => res.json())
      .then(
        this.setMyInfo,
        err => {
          console.log("Failed to fetch user data")
          alert("something went wrong")
        }
      )
  }

  setMyInfo(res) {
    if (res.success == false) {
      console.log("Failed to fetch user data")
      alert("something went wrong")
      return;
    }
    const user = res.result
    this.setState({
      user: {
        name: user.name,
        coins: user.hold_coin,
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

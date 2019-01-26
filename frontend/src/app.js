import React from 'react';
import { connect } from 'react-redux';

import LoginPage from './pages/login';
import AccountPage from './pages/account';
import css from './index.css';

class App extends React.Component {
  render() {
    if (this.props.needLogin) {
      return (
        <div id={css.app}>
          <LoginPage />
        </div>
      );
    } else {
      return (
        <div id={css.app}>
          <AccountPage />
        </div>
      );
    }
  }
}

function mapStateToProps(state) {
  return {
    needLogin: state.login.accessToken == "",
  }
}

export default connect(
  mapStateToProps,
  null,
)(App)

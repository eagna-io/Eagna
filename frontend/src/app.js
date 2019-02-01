import React from 'react';
import {
  BrowserRouter as Router,
  Route,
  Switch,
  Redirect
} from "react-router-dom";
import { connect } from 'react-redux';
import styled from 'styled-components';

import LoginPage from './pages/login';
import AccountPage from './pages/account';

class App extends React.Component {
  render() {
    return (
      <Router>
        <Switch>
          <Route path="/login" component={LoginPage} />
          <Route path="/me" component={AccountPage} />
        </Switch>
      </Router>
    )
  }
}

export default connect(
  null,
  null,
)(App)

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
import MarketPage from './pages/market';

class App extends React.Component {
  render() {
    return (
      <Router>
        <Switch>
          <Route path="/me" component={AccountPage} />
          <Route path="/market/:id" component={MarketPage} />
          <Route path="/login" component={LoginPage} />
          <Route component={LoginPage} />
        </Switch>
      </Router>
    )
  }
}

export default connect(
  null,
  null,
)(App)

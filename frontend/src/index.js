import React from 'react';
import ReactDOM from 'react-dom';
import LoginPage from './pages/login';
import AccountPage from './pages/account';
import css from './index.css';

class App extends React.Component {
  render() {
    return (
      <div id={css.app}>
        <AccountPage name="alice" coins="100"/>
      </div>
    );
  }
}

ReactDOM.render(<App />, document.getElementById('app'))

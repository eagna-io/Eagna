import React from 'react';
import ReactDOM from 'react-dom';
import LoginPage from './pages/login';
import css from './index.css';

class App extends React.Component {
  render() {
    return (
      <div id={css.app}>
        <LoginPage />
      </div>
    );
  }
}

ReactDOM.render(<App />, document.getElementById('app'))

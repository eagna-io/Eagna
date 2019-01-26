import React from 'react';
import { connect } from 'react-redux';

import { requestMe } from '../actions';
import css from './account.css';

class AccountPage extends React.Component {
  constructor(props) {
    super(props);
    const token = this.props.token;
    this.props.requestMe(token);
  }

  render() {
    const name = this.props.name;
    const coins = this.props.coins;
    return (
      <div className={css.container}>
        <div className={css.row}>
          <div className={css.key}>Name</div>
          <div className={css.val}>{name}</div>
        </div>
        <div className={css.row}>
          <div className={css.key}>Coins</div>
          <div className={css.val}>{coins}<span className={css.coins}>coins</span></div>
        </div>
        <div className={css.row}>
          <div className={css.key}>Activities</div>
        </div>
        <table className={css.markets}>
          <thead>
            <tr>
              <th>Market</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>Who will win the Tennis Australian Open 2018?</td>
              <td>Opened</td>
            </tr>
          </tbody>
        </table>
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    token: state.login.accessToken,
    name: state.me.name,
    coins: state.me.coins,
    markets: state.me.markets,
  }
}

function mapDispatchToProps(dispatch) {
  return {
    requestMe: token => dispatch(requestMe(token))
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(AccountPage)

import React from 'react';
import { connect } from 'react-redux';
import { Redirect } from 'react-router-dom';

import { requestMe } from '../actions';
import css from './account.css';

class AccountPage extends React.Component {
  constructor(props) {
    super(props);
    if (this.props.token != null) {
      this.props.requestMe(this.props.token);
    }
  }

  render() {
    if (this.props.token == null) {
      return <Redirect to="/login" />
    }
    const name = this.props.name == null ? "-" : this.props.name;
    const coins = this.props.coins == null ? 0 : this.props.coins;
    const markets = this.props.markets == null ? [] : this.props.markets;
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
          {markets.map(market =>
          <tbody key={market.title}>
            <tr>
              <td>{market.title}</td>
              <td>{market.status}</td>
            </tr>
          </tbody>
          )}
        </table>
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    isRequesting: state.pages.me.isRequesting,
    token: state.me.accessToken,
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

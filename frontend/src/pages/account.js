import React from 'react';
import css from './account.css';

export default class AccountPage extends React.Component {
  render() {
    const user = this.props.user;
    const name = user === undefined ? "-" : user.name;
    const coins = user === undefined ? "-" : user.coins;
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

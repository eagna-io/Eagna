import React from 'react';
import css from './account.css';

export default class AccountPage extends React.Component {
  render() {
    return (
      <div className={css.container}>
        <div className={css.row}>
          <div className={css.key}>Name</div>
          <div className={css.val}>{this.props.name}</div>
        </div>
        <div className={css.row}>
          <div className={css.key}>Coins</div>
          <div className={css.val}>{this.props.coins}<span className={css.coins}>coins</span></div>
        </div>
        <div className={css.row}>
          <div className={css.key}>Activities</div>
        </div>
        <table className={css.markets}>
          <th>Market</th><th>Status</th>
          <tr>
            <td>Who will win the Tennis Australian Open 2018?</td>
            <td>Opened</td>
          </tr>
        </table>
      </div>
    );
  }
}

import React from 'react';
import sha256 from 'js-sha256';
import css from './login.css';

export default class LoginPage extends React.Component {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
    this.onPressLogin = this.onPressLogin.bind(this);
    this.handleApiResponse = this.handleApiResponse.bind(this);
    this.state = {
      userName: "hoge",
      password: "",
    };
  }

  handleChange(event) {
    const target = event.target;
    this.setState({
      [target.name]: target.value
    })
  }

  onPressLogin(event) {
    event.preventDefault(); 
    const name = this.state.userName;
    const pass = sha256(this.state.password);
    console.log(this.state)
    fetch("http://localhost:8099/login?user="+name+"&pass="+pass)
      .then(res => res.json())
      .then(
        this.handleApiResponse, // on success
        err => {
          console.err("Login error : " + err)
          alert("something went wrong")
        }
      )
  }

  handleApiResponse(res) {
    if (res.success == false) {
      alert("Login failed")
      return
    }
    alert("login success!!!!!")
  }

  render() {
    return (
      <div className={css.container}>
        <div className={css.title}>ROHAN MARKET</div>
        <input
          className={css.input}
          type="text"
          name="userName"
          placeholder="User Name"
          onChange={this.handleChange} />
        <input
          className={css.input}
          type="text"
          name="password"
          placeholder="Password"
          onChange={this.handleChange} />
        <button
          className={css.button}
          onClick={this.onPressLogin}>Login</button>
      </div>
    );
  }
}

import React, { useContext } from 'react';
import { RouterContext } from 'src/context';


export const Login = "Login";
export const Account = "Account";
export const Market = "Market";

export default class Router {
  constructor(setPage) {
    this.setPage = setPage;
  }

  redirectTo(nextPage) {
    history.pushState(null, "", nextPage.path);
    this.setPage(nextPage);
  }
}

export function LoginPage() {
  return {
    name: Login,
    path: "/login",
    params: null,
  }
}

export function AccountPage() {
  return {
    name: Account,
    path: "/me",
    params: null,
  }
}

export function MarketPage(id) {
  return {
    name: Market,
    path: `/market/${id}`,
    params: {id}
  }
}

export function pageFromPath(path) {
  const loginPath = /^\/login/;
  const accountPath = /^\/me$/;
  const marketPath = /^\/market\/(\d+)$/;

  if (loginPath.test(path)) {
    return LoginPage();
  }

  if (accountPath.test(path)) {
    return AccountPage();
  }

  let match = marketPath.exec(path);
  if (match) {
    return MarketPage(match[1]);
  }
}


export function Link(props) {
  const to = props.to;
  const router = useContext(RouterContext);
  return (
    <a
      className={props.className}
      href={to.path}
      onClick={e => {
        e.preventDefault();
        router.redirectTo(to);
      }}>
      {props.children}
    </a>
  );
}

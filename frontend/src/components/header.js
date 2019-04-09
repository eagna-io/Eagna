import React, { useState, useContext } from 'react';
import styled from 'styled-components';
import onClickOutside from 'react-onclickoutside';
import { Link } from 'react-router-dom';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faUserCircle, faCaretDown } from '@fortawesome/free-solid-svg-icons';

import { AccessTokenContext } from 'src/context';

export default function Header(props) {
  const history = props.history;

  const render = () => (
    <Container className={props.className}>
      <Link to="/"><Logo src="/img/logo.png" /></Link>
      <ProfileDropdown history={history}/>
    </Container>
  );

  const Container = styled.div`
    display: flex;
    width: 100vw;
    height: 60px;
    padding: 0 30px;
    background-color: #1c384e;
    border-bottom: 1px solid #979797;
    justify-content: space-between;
    align-items: center;
  `;

  const Logo = styled.img`
    display: block;
    height: 50px;
    margin-left: 30px;
  `;

  return render();
}

function ProfileDropdown(props) {
  const history = props.history;
  const [showMenu, setShowMenu] = useState(false);
  const {accessToken, setAccessToken} = useContext(AccessTokenContext);

  const toggleDropdown = e => {
    e.preventDefault();
    setShowMenu(!showMenu);
  };

  const signOut = () => {
    setAccessToken(null);
    history.push("/login");
  }

  const signIn = () => {
    history.push("/login");
  }

  const clickOutsideConfig = {
    handleClickOutside: () => () => setShowMenu(false),
  };

  function Dropdown(props) {
    return <div>{props.children}</div>;
  }

  const Container = onClickOutside(Dropdown, clickOutsideConfig);

  const isLogin = accessToken !== null;

  const render = () => (
    <Container className={props.className}>
      <ProfileButton onClick={toggleDropdown}>
        <FontAwesomeIcon icon={faUserCircle} size="3x" />
        <DropdownCaret>
          <FontAwesomeIcon icon={faCaretDown} size="lg" />
        </DropdownCaret>
      </ProfileButton>
      <ProfileMenu show={showMenu}>
        {
          isLogin ? (
            <>
            <MenuItem>
              <MenuItemLink to="/me">Account Page</MenuItemLink>
            </MenuItem>
            <Line />
            <MenuItem>
              <Signout onClick={signOut}>Sign out</Signout>
            </MenuItem>
            </>
          ) : (
            <MenuItem>
              <Signin onClick={signIn}>Sign in</Signin>
            </MenuItem>
          )
        }
      </ProfileMenu>
    </Container>
  );

  const ProfileButton = styled.button`
    display: block;
    height: 40px;
    line-height: 40px;
    color: white;
    background-color: rgba(0,0,0,0);
    border: none;
    padding: 0;
    cursor: pointer;
  `;

  const DropdownCaret = styled.span`
    display: inline-block;
    margin-left: 5px;
    vertical-align: top;
  `;

  const ProfileMenu = styled.div`
    position: absolute;
    display: ${props => props.show ? "block" : "none"};
    top: 50px;
    right: 20px;
    background-color: white;
    border: 1px solid rgba(27,31,35,.15);
    border-radius: 4px;
    box-shadow: 0 3px 12px rgba(27,31,35,.15);
  `;

  const Line = styled.hr`
    border: 1px solid #e1e4e8;
    width: 90%;
    margin: 5px auto;
  `;

  const MenuItem = styled.div`
    padding: 10px;
    padding-right: 30px;
    font-size: 13px;
    color: #586069;
  `;

  const MenuItemLink = styled(Link)`
    text-decoration: none;
     &:visited {
      color: #586069;
     }
  `;

  const Signout = styled.div`
    cursor: pointer;
  `;

  const Signin = styled.div`
    cursor: pointer;
  `;

  return render();
}


;

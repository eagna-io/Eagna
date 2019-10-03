import React, { FC, useState } from "react";
import styled from "styled-components";
import OutsideClickHandler from "react-outside-click-handler";
import { Link, withRouter } from "react-router-dom";
import { useSelector } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faUserCircle, faCaretDown } from "@fortawesome/free-solid-svg-icons";
import { History } from "history";
import * as firebase from "firebase/app";
import "firebase/auth";

import { User } from "models/user";
import { pc } from "app/components/responsive";
import { RootState } from "app/redux";

interface HeaderProps {
  history: History;
}

const Header: FC<HeaderProps> = ({ history }) => {
  const user = useSelector((state: RootState) => state.user.user);
  return (
    <Container>
      <Link to="/">
        <Logo src="/img/logo.png" />
      </Link>
      <ProfileDropdownContainer>
        <ProfileDropdown history={history} user={user} />
      </ProfileDropdownContainer>
    </Container>
  );
};

export default withRouter(Header);

const Container = styled.div`
  width: 100vw;
  height: 60px;
  padding: 0 30px;
  background-color: #1c384e;
`;

const Logo = styled.img`
  display: inline-block;
  position: absolute;
  height: 50px;
  top: 5px;
  left: 30px;
`;

const ProfileDropdownContainer = styled.div`
  position: absolute;
  top: 15px;
  right: 30px;
`;

interface ProfileDropdownProps {
  history: History;
  user: User | null | undefined;
}

const ProfileDropdown: FC<ProfileDropdownProps> = ({ history, user }) => {
  const [showMenu, setShowMenu] = useState(false);

  const toggleDropdown = () => {
    setShowMenu(!showMenu);
  };

  const signOut = () => {
    firebase
      .auth()
      .signOut()
      .then(function() {
        history.push("/");
      });
  };

  const signIn = () => {
    history.push("/login");
  };

  return (
    <OutsideClickHandler
      onOutsideClick={() => {
        setShowMenu(false);
      }}
    >
      <ProfileButton onClick={toggleDropdown}>
        <FontAwesomeIcon icon={faUserCircle} size="3x" />
        <DropdownCaret>
          <FontAwesomeIcon icon={faCaretDown} size="lg" />
        </DropdownCaret>
      </ProfileButton>
      <ProfileMenu show={showMenu}>
        {user instanceof User ? (
          <>
            <MenuItem>
              <MenuItemLink to="/account">{user.name}</MenuItemLink>
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
        )}
      </ProfileMenu>
    </OutsideClickHandler>
  );
};

const ProfileButton = styled.button`
  display: block;
  height: 40px;
  line-height: 40px;
  color: white;
  background-color: rgba(0, 0, 0, 0);
  border: none;
  padding: 0;
  cursor: pointer;
`;

const DropdownCaret = styled.span`
  display: inline-block;
  margin-left: 5px;
  vertical-align: top;
`;

const ProfileMenu = styled("div")<{ show: boolean }>`
  position: absolute;
  display: ${props => (props.show ? "block" : "none")};
  width: 100px;
  top: 50px;
  right: 10px;
  z-index: 1000;
  background-color: white;
  border: 1px solid rgba(27, 31, 35, 0.15);
  border-radius: 4px;
  box-shadow: 0 3px 12px rgba(27, 31, 35, 0.15);

  ${pc(`
      right: 20px;
    `)}
`;

const Line = styled.hr`
  border: 1px solid #e1e4e8;
  width: 90%;
  margin: 5px auto;
`;

const MenuItem = styled.div`
  padding: 10px;
  font-size: 11px;
  color: #586069;

  ${pc(`
      padding-right: 30px;
      font-size: 13px;
    `)}
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

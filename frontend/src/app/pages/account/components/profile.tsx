import React from 'react';
import styled from 'styled-components';

import {User} from 'models/user';
import {pc} from 'app/components/responsive';
import * as table from 'app/components/table';

const ProfileComponent: React.FC<{user: User}> = ({user}) => {
  return (
    <Container>
      <table.Table>
        <table.Header>
          <table.Cell>
            <TableTitle>プロフィール</TableTitle>
          </table.Cell>
        </table.Header>
        <table.Body>
          <table.Row>
            <table.Cell3>
              <LeftItem>名前</LeftItem>
            </table.Cell3>
            <table.Cell7>
              <RightItem>{user.name}</RightItem>
            </table.Cell7>
          </table.Row>
          <table.Row>
            <table.Cell3>
              <LeftItem>メール</LeftItem>
            </table.Cell3>
            <table.Cell7>
              <RightItem>{user.email}</RightItem>
            </table.Cell7>
          </table.Row>
        </table.Body>
      </table.Table>
    </Container>
  );
};

export default ProfileComponent;

const Container = styled.div`
  width: 100%;

  ${pc(`
    width: 330px;
  `)}
`;

const TableTitle = styled.h3`
  margin: 0;
  padding: 0;
  font-size: 12px;
  font-weight: normal;
`;

const LeftItem = styled.div`
  text-align: left;
  font-size: 14px;
`;

const RightItem = styled.div`
  width: 100%;
  text-align: right;
  font-size: 14px;
  font-weight: bold;
`;

import React, {FC} from 'react';
import styled from 'styled-components';

import * as table from 'components/table';
import User from 'models/user';

const Component: FC<{me: User}> = ({me}) => {
  return (
    <Table>
      <table.Header>
        <table.Cell>プロフィール</table.Cell>
      </table.Header>
      <table.Body>
        <table.Row>
          <table.Cell3>
            <LeftItem>名前</LeftItem>
          </table.Cell3>
          <table.Cell7>
            <RightItem>{me.name}</RightItem>
          </table.Cell7>
        </table.Row>
        <table.Row>
          <table.Cell3>
            <LeftItem>メール</LeftItem>
          </table.Cell3>
          <table.Cell7>
            <RightItem>{me.email}</RightItem>
          </table.Cell7>
        </table.Row>
      </table.Body>
    </Table>
  );
};

export default Component;

const Table = styled(table.Table)`
  width: 100%;
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
  text-align: right;
`;

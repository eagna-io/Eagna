import React, {FC, useState, useCallback, useContext} from 'react';
import styled from 'styled-components';

const TableContext = React.createContext<{
  striped: boolean;
}>({
  striped: true,
});

interface TableProps {
  striped?: boolean;
  className?: string;
}

export const Table: FC<TableProps> = ({striped, className, children}) => {
  const context = {
    striped: striped || false,
  };

  return (
    <TableContainer className={className}>
      <TableContext.Provider value={context}>{children}</TableContext.Provider>
    </TableContainer>
  );
};

const TableContainer = styled.table`
  display: block;
  position: relative;
  width: 100%;
  table-layout: fixed;
  border-radius: 4px;
  border: 1px solid #d1d5da;
  border-spacing: 0;
  border-collapse: collapse;
  overflow: scroll;
`;

export const Header: FC<{className?: string}> = ({className, children}) => {
  return (
    <HeaderContainer className={className}>
      <Row>{children}</Row>
    </HeaderContainer>
  );
};

const HeaderContainer = styled.thead`
  display: block;
  width: 100%;
  background-color: #f6f8fa;
  color: #586069;
  border-bottom: 1px solid #d1d5da;
`;

export const Body: FC = ({children}) => {
  return <BodyContainer>{children}</BodyContainer>;
};

const BodyContainer = styled.tbody`
  display: block;
  width: 100%;
  padding: 0;
  margin: 0;
  list-style-type: none;
  overflow: scroll;
`;

export const Row: FC = ({children}) => {
  const {striped} = useContext(TableContext);

  return <RowContainer striped={striped}>{children}</RowContainer>;
};

const RowContainer = styled('tr')<{striped: boolean}>`
  display: block;
  width: 100%;

  &:nth-child(even) {
    background-color: ${props => (props.striped ? '#F9F9F9' : 'white')};
  }
`;

export const Cell = styled.td`
  display: inline-block;
  padding: 3% 3% 3% 3%;

  &:last-child {
    padding-right: 3%;
  }
`;

export const Cell1 = styled(Cell)`
  width: 10%;
`;

export const Cell2 = styled(Cell)`
  width: 20%;
`;

export const Cell3 = styled(Cell)`
  width: 30%;
`;

export const Cell4 = styled(Cell)`
  width: 40%;
`;

export const Cell5 = styled(Cell)`
  width: 50%;
`;

export const Cell6 = styled(Cell)`
  width: 60%;
`;

export const Cell7 = styled(Cell)`
  width: 70%;
`;

export const Cell8 = styled(Cell)`
  width: 80%;
`;

export const Cell9 = styled(Cell)`
  width: 90%;
`;

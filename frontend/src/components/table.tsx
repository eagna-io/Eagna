import React, {FC, useState, useCallback} from 'react';
import styled from 'styled-components';

interface TableProps {
  maxHeight?: number;
  className?: string;
}

export const Table: FC<TableProps> = ({maxHeight, className, children}) => {
  const [height, setHeight] = useState<number | undefined>();

  const ref = useCallback(node => {
    if (node !== null) {
      setHeight(node.getBoundingClientRect().height);
    }
  }, []);

  const Container = styled.div`
    border-radius: 4px;
    border: 1px solid #d1d5da;
  `;

  const Inner = styled('table')`
    display: block;
    width: 100%;
    height: ${maxHeight === undefined ||
    height === undefined ||
    height < maxHeight
      ? 'auto'
      : `${maxHeight}px`};
    table-layout: fixed;
    border-spacing: 0;
    border-collapse: collapse;
    overflow: hidden;
  `;

  return (
    <Container className={className}>
      <Inner ref={ref}>{children}</Inner>
    </Container>
  );
};

export const Header: FC<{className?: string}> = ({className, children}) => {
  const Container = styled.thead`
    display: block;
    width: 100%;
    background-color: #f6f8fa;
    color: #586069;
    border-bottom: 1px solid #d1d5da;
  `;

  return (
    <Container className={className}>
      <Row>{children}</Row>
    </Container>
  );
};

export const Body = styled.tbody`
  display: block;
  width: 100%;
  height: calc(100% - 49px);
  padding: 0;
  margin: 0;
  list-style-type: none;
  overflow: scroll;
`;

interface RowProps {
  striped?: boolean;
  className?: string;
}

export const Row = styled('tr')<{striped?: boolean}>`
  display: block;
  width: 100%;

  &:nth-child(even) {
    background-color: ${props => (props.striped ? '#F9F9F9' : 'rgba(0,0,0,0)')};
  }
`;

export const Cell = styled('td')<{
  right?: boolean;
  bold?: boolean;
  small?: boolean;
  large?: boolean;
}>`
  display: inline-block;
  min-height: 48px;
  padding: 14px 40px 14px 16px;
  font-size: ${props => (props.small ? '12px' : props.large ? '16px' : '14px')};
  font-weight: ${props => (props.bold ? 'bold' : 'normal')};
  text-align: ${props => (props.right ? 'right' : 'left')};

  &:last-child {
    padding-right: 16px;
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

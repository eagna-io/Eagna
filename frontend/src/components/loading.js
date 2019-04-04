import React from 'react';
import styled from 'styled-components';

export default function Loading(props) {
  const loading = props.loading;

  if (loading) {
    return (
      <View className={props.className}>
        <Spinner className="fas fa-spinner fa-spin fa-5x"></Spinner>
      </View>
    );
  } else {
    return null;
  }
}

const View = styled.div`
  position: fixed;
  z-index: 10;
  width: 100vw;
  height: 100vh;
  top: 0;
  left: 0;
  background-color: rgba(0, 0, 0, 0.3);
`;

const Spinner = styled.i`
  margin-left: calc(50vw - 45px);
  margin-top: calc(50vh - 45px);
  color: white;
`;

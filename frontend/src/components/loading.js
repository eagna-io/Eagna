import React from 'react';
import styled from 'styled-components';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSpinner } from '@fortawesome/free-solid-svg-icons';

export default function Loading(props) {
  const loading = props.loading;

  if (loading) {
    return (
      <View className={props.className}>
        <Spinner icon={faSpinner} size="5x" spin />
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

const Spinner = styled(FontAwesomeIcon)`
  margin-left: calc(50vw - 45px);
  margin-top: calc(50vh - 45px);
  color: white;
`;

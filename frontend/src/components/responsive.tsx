import React, {FC, useState} from 'react';
import styled from 'styled-components';

interface ResponsiveProps {
  renderPc: () => React.ReactNode;
  renderTablet: () => React.ReactNode;
  renderMobile: () => React.ReactNode;
}

export const Responsive: FC<ResponsiveProps> = ({
  renderPc,
  renderTablet,
  renderMobile,
}) => {
  const [VW, setVW] = useState(window.innerWidth);

  const onWindowResize = () => {
    setVW(window.innerWidth);
  };

  window.onresize= onWindowResize;

  return (
    <View>
      {VW < 768 ? renderMobile() : VW < 980 ? renderTablet() : renderPc()}
    </View>
  );
};

export default Responsive;

const View = styled.div`
  width: 100vw;
  height: 100vh;
`;

import React from "react";
import styled from "styled-components";

interface Props {
  marketTitle: string;
}

const Contents: React.FC<Props> = ({ marketTitle }) => {

  return (
    <Container>
      <MarketTitle>{marketTitle}</MarketTitle>
    </Container>
  );
};

export default Contents;

const Container = styled.div`
  position: relative;
  background-color: #242423;
  padding: 0 20px 20px;
`;

const MarketTitle = styled.div`
  font-size: 18px;
  color: #BB86FC;
  font-weight: 300;
`;

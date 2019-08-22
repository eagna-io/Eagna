import React from 'react';
import styled from 'styled-components';

import {pc} from 'app/components/responsive';

interface Props {
  title: string;
}

const Section: React.FC<Props> = ({title, children}) => {
  return (
    <Container>
      <SectionTitle>{title}</SectionTitle>
      <Contents>{children}</Contents>
    </Container>
  );
};

export default Section;

const Container = styled.div`
  width: 100%;
  padding: 30px 0;

  ${pc(`
    padding: 50px 0;
  `)}
`;

const SectionTitle = styled.h3`
  margin: 0;
  padding: 0;
  font-size: 16px;

  ${pc(`
  font-size: 26px;
  `)}
`;

const Contents = styled.div`
  width: 100%;
  padding: 20px;
  padding-bottom: 0;

  ${pc(`
    padding: 30px;
  `)}
`;

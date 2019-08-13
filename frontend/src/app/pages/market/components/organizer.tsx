import React from 'react';
import styled from 'styled-components';

import {Organizer} from 'models/organizer';
import Section from './section_skelton';

interface Props {
  organizer: Organizer;
}

const OrganizerComponent: React.FC<Props> = ({organizer}) => {
  return (
    <Section title="マーケット作成者">
      <Sumbnail src={organizer.sumbnailUrl} />
      <Name>{organizer.name}</Name>
    </Section>
  );
};

export default OrganizerComponent;

const Sumbnail = styled.img`
  display: block;
  width: 125px;
  margin: 0 auto;
  margin-top: 30px;
`;

const Name = styled.h4`
  margin-top: 20px;
  padding-left: 20px;
  font-weight: bold;
  font-size: 14px;
`;

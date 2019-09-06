import React from "react";
import styled from "styled-components";

import { Eagna } from "models/organizer";
import { pc } from "app/components/responsive";

import Section from "./section_skelton";

const OrganizerComponent: React.FC = () => {
  const organizer = Eagna;

  return (
    <Section title="マーケット作成者">
      <Sumbnail src={organizer.thumbnailUrl} />
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

  ${pc(`
    width: 300px;
  `)}
`;

const Name = styled.h4`
  margin-top: 20px;
  padding-left: 20px;
  font-weight: bold;
  font-size: 14px;

  ${pc(`
    font-size: 25px;
  `)}
`;

import React from "react";
import styled from "styled-components";
import ReactMarkdown from "react-markdown";

import { pc } from "app/components/responsive";

import { useMarket } from "./data_provider";
import Section from "./section_skelton";

const DescComponent: React.FC = () => {
  const { market } = useMarket();

  return (
    <Section title="概要">
      <StyledReactMarkdown
        source={market.attrs.description}
        linkTarget="_blank"
      />
    </Section>
  );
};

export default DescComponent;

const StyledReactMarkdown = styled(ReactMarkdown)`
  width: 100%;
  overflow: hidden;
  font-size: 12px;

  ${pc(`
    font-size: 16px;
  `)}

  & a {
    text-decoration: underline;
  }
`;

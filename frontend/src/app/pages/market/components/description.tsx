import React from "react";
import styled from "styled-components";
import ReactMarkdown from "react-markdown";

import { pc } from "app/components/responsive";

import { useMarket } from "./data_provider";
import Section from "./section_skelton";

const DescComponent: React.FC = () => {
  const { market } = useMarket();
  const [showMore, setShowMore] = React.useState(false);

  return (
    <Section title="概要">
      <StyledReactMarkdown
        showMore={showMore}
        source={market.attrs.description}
        linkTarget="_blank"
      />
      {!showMore ? (
        <ReadMoreButton onClick={() => setShowMore(true)}>
          もっと読む
        </ReadMoreButton>
      ) : null}
    </Section>
  );
};

export default DescComponent;

const StyledReactMarkdown = styled(ReactMarkdown)<{ showMore: boolean }>`
  width: 100%;
  height: ${props => (props.showMore ? "auto" : "100px")};
  overflow: hidden;
  font-size: 12px;

  ${pc(`
    font-size: 16px;
  `)}

  & a {
    text-decoration: underline;
  }
`;

const ReadMoreButton = styled.button`
  font-size: 14px;
  color: #b4b4b4;
  text-decoration: underline;
`;

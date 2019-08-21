import React from 'react';
import styled from 'styled-components';
import ReactMarkdown from 'react-markdown';

import {pc} from 'app/components/responsive';

import Section from './section_skelton';

interface Props {
  desc: string;
}

const DescComponent: React.FC<Props> = ({desc}) => {
  const [showMore, setShowMore] = React.useState(false);

  return (
    <Section title="概要">
      <StyledReactMarkdown
        showMore={showMore}
        source={desc}
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

const StyledReactMarkdown = styled(ReactMarkdown)<{showMore: boolean}>`
  width: 100%;
  height: ${props => (props.showMore ? 'auto' : '100px')};
  overflow: hidden;
  font-size: 12px;

  ${pc(`
    font-size: 16px;
  `)}
`;

const ReadMoreButton = styled.button`
  font-size: 14px;
  color: #b4b4b4;
  text-decoration: underline;
`;

import React from "react";
import styled from "styled-components";

import {
  ChoiceBlue,
  MainRed,
  WhiteBaseColor,
  VoteRateBackGround
} from "app/components/color";

import { ReactComponent as CorrectIcon } from "../atoms/images/correct.svg";
import { Poll } from "../../models";

interface Props {
  poll: Poll;
  selected?: string;
  onSelected: (choice: string) => void;
}

export const ChoiceList: React.FC<Props> = ({ poll, selected, onSelected }) => {
  if (poll.status === "open") {
    return (
      <Container>
        {Object.entries(poll.choices).map(([title, color]) => (
          <Choice
            title={title}
            color={color}
            selected={selected === title}
            onSelected={() => {
              onSelected(title);
            }}
          />
        ))}
      </Container>
    );
  } else {
    return (
      <Container>
        {Object.entries(poll.choices).map(([title, color]) => (
          <Choice
            title={title}
            color={color}
            selected={selected === title}
            onSelected={() => {
              onSelected(title);
            }}
            disabled
            correct={poll.resolved === title}
          />
        ))}
      </Container>
    );
  }
};

const Container = styled.div`
  width: 100%;
  overflow: scroll;
`;

interface ChoiceProps {
  title: string;
  color: string;
  selected: boolean;
  onSelected: () => void;
  disabled?: boolean;
  voteRate?: number;
  correct?: boolean;
}

const Choice: React.FC<ChoiceProps> = ({
  title,
  color,
  selected,
  onSelected,
  disabled,
  voteRate,
  correct
}) => {
  return (
    <ChoiceContainer>
      <OutcomeIcon correct={correct} />
      <ChoiceButton
        color={color}
        selected={selected}
        onClick={() => onSelected()}
        disabled={disabled}
      >
        {voteRate !== undefined ? (
          <>
            <VoteRate voteRate={voteRate} />
            <RateValue color={color} selected={selected}>
              {voteRate}%
            </RateValue>
          </>
        ) : null}
        <Choicetitle color={color} selected={selected}>
          {title}
        </Choicetitle>
      </ChoiceButton>
    </ChoiceContainer>
  );
};

const ChoiceContainer = styled.div`
  display: flex;
  position: relative;
  margin-bottom: 24px;
`;

const OutcomeIcon = styled(CorrectIcon)<{ correct?: boolean }>`
  position: absolute;
  left: 0;
  width: 40px;
  height: 48px;
  display: ${props => (props.correct ? "block" : "none")};
`;

const ChoiceButton = styled.button<{ color: string; selected: boolean }>`
  position: relative;
  width: 200px;
  height: 48px;
  border-radius: 24px;
  margin: 0px auto;
  padding: 4px 8px;
  border: solid 3px ${props => props.color};
  background-color: ${props =>
    props.selected ? props.color : WhiteBaseColor.hex};
  font-size: 14px;
  font-weight: 600;

  &:disabled {
    opacity: ${props => (props.selected ? null : "0.5")};
    cursor: default;
  }
`;

const VoteRate = styled.div<{ voteRate: number }>`
  position: absolute;
  top: 0px;
  left: 0px;
  width: ${props => props.voteRate}%;
  height: 42px;
  border-radius: 24px;
  background-color: ${VoteRateBackGround.hexWithOpacity(0.5)};
`;

const Choicetitle = styled.div<{ color: string; selected: boolean }>`
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit-transform: translateY(-50%) translateX(-50%);
  width: 180px;
  text-align: left;
  color: ${props => (props.selected ? WhiteBaseColor.hex : props.color)};
`;

const RateValue = styled.div<{ color: string; selected: boolean }>`
  position: absolute;
  top: 50%;
  right: 0;
  transform: translateY(-50%) translateX(-50%);
  -webkit-transform: translateY(-50%) translateX(-50%);
  color: ${props => (props.selected ? WhiteBaseColor.hex : props.color)};
`;

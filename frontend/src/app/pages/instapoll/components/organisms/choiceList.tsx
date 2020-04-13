import React from "react";
import styled from "styled-components";

import { WhiteBaseColor, VoteRateBackGround } from "app/components/color";

import { ReactComponent as CorrectIcon } from "../atoms/images/correct.svg";
import { ReactComponent as WrongIcon } from "../atoms/images/wrong.svg";
import { Poll } from "model/poll";

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
            isOpen={true}
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
            isOpen={false}
            disabled
            correct={poll.resolved === title}
            voteRate={
              Math.floor((poll.stats!.votePerChoice[title] / poll.stats!.totalVotes) * 100)
            }
          />
        ))}
      </Container>
    );
  }
};

const Container = styled.div`
  width: 100%;
  height: 127px;
  overflow-y: scroll;
  ::-webkit-scrollbar {
    -webkit-appearance: none;
    width: 7px;
  }
  ::-webkit-scrollbar-thumb {
      border-radius: 4px;
      background-color: rgba(0,0,0,.5);
      box-shadow: 0 0 1px rgba(255,255,255,.5);
  }
`;

interface ChoiceProps {
  title: string;
  color: string;
  selected: boolean;
  onSelected: () => void;
  disabled?: boolean;
  voteRate?: number;
  correct?: boolean;
  isOpen: boolean;
}

const Choice: React.FC<ChoiceProps> = ({
  title,
  color,
  selected,
  onSelected,
  disabled,
  voteRate,
  correct,
  isOpen
}) => {
/* 
【MEMO：正誤アイコン表示の方針】
正解 => <Correct />
不正解 && 自分が選んだ => <Wrong />
不正解 && 自分が選んでいない => null
*/
  return (
    <ChoiceContainer>
      { correct ? <Correct /> : selected ? <Wrong isOpen={isOpen} /> : null }
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
  &:last-child {
    margin-bottom: 0px;
  }
`;

const Correct = styled(CorrectIcon)`
  position: absolute;
  left: 0;
  width: 22px;
  height: 50px;
`;

const Wrong = styled(WrongIcon)<{ isOpen: boolean }>`
  display: ${ props => props.isOpen ? "none" : "block" };
  position: absolute;
  left: 0;
  width: 22px;
  height: 50px;
`;

const ChoiceButton = styled.button<{ color: string; selected: boolean }>`
  position: relative;
  width: 196px;
  height: 50px;
  border-radius: 24px;
  margin: 0px auto;
  padding: 4px 8px;
  border: solid 4px ${props => props.color};
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
  border-radius: 24px 0 0 24px;
  background-color: ${VoteRateBackGround.hexWithOpacity(0.5)};
`;

const Choicetitle = styled.div<{ color: string; selected: boolean }>`
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translateY(-50%) translateX(-50%);
  -webkit-transform: translateY(-50%) translateX(-50%);
  width: 180px;
  padding: 0 40px 0 10px;
  text-align: left;
  color: ${props => (props.selected ? WhiteBaseColor.hex : props.color)};
  overflow: hidden;
  overflow-wrap: break-word;
  word-wrap: break-word;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
`;

const RateValue = styled.div<{ color: string; selected: boolean }>`
  position: absolute;
  top: 50%;
  right: 0;
  transform: translateY(-50%) translateX(-50%);
  -webkit-transform: translateY(-50%) translateX(-50%);
  color: ${props => (props.selected ? WhiteBaseColor.hex : props.color)};
`;

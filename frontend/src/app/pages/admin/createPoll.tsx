import React from "react";
import styled from "styled-components";
import * as D from "@mojotech/json-type-validation";

import * as color from "app/components/color";
import * as pollApi from "infra/http/poll";
import * as storage from "infra/storage";

import { AdminTemplate } from "./components/template/admin";

export const CreatePoll: React.FC = () => {
  const [accessToken] = React.useState(storage.getAdminAccessToken);
  const [contestId, setContestId] = React.useState("");
  const [title, setTitle] = React.useState("");
  const [durationSec, setDurationSec] = React.useState(30);
  const [choices, setChoices] = React.useState<
    { name: string; color: string; idx: number }[]
  >([{ name: "", color: colorOfIdx(0).hex, idx: 0 }]);

  return (
    <AdminTemplate>
      <QuestionContainer>
        <QuestionTag>コンテストID</QuestionTag>
        <QuestionInput
          value={contestId}
          onChange={e => {
            setContestId(e.target.value);
          }}
        />
      </QuestionContainer>
      <QuestionContainer>
        <QuestionTag>問題文</QuestionTag>
        <QuestionInput
          onChange={e => {
            setTitle(e.target.value);
          }}
        />
      </QuestionContainer>
      <QuestionContainer>
        <QuestionTag>期間（秒数）</QuestionTag>
        <QuestionInput
          onChange={e => {
            setDurationSec(Number(e.target.value));
          }}
        />
      </QuestionContainer>
      <ChoiceContainer>
        <ChoiceList>
          <ChoiceTag>選択肢</ChoiceTag>
          <ChoiceUl>
            {choices.map(({ name, color }, i) => (
              <Choiceitem key={i}>
                <ChoiceInput
                  placeholder="選択肢名"
                  onChange={e => {
                    const newChoices = [...choices];
                    newChoices[i].name = e.target.value;
                    setChoices(newChoices);
                  }}
                />
              </Choiceitem>
            ))}
          </ChoiceUl>
        </ChoiceList>
        <AddChoice
          onClick={() => {
            const nextColor = colorOfIdx(choices.length);
            setChoices([
              ...choices,
              { name: "", color: nextColor.hex, idx: choices.length }
            ]);
          }}
        >
          選択肢を追加
        </AddChoice>
        <RemoveChoice
          onClick={() => {
            const n = [...choices];
            n.pop();
            setChoices(n);
          }}
        >
          選択肢を削除
        </RemoveChoice>
      </ChoiceContainer>
      <CreatePollButton
        onClick={async () => {
          if (!accessToken) {
            alert("ログインが必要です");
            return;
          }
          const res = await pollApi.post({
            contestId,
            title,
            durationSec,
            choices,
            accessToken
          });
          alert(JSON.stringify(res));
          setTitle("");
          setChoices([{ name: "", color: colorOfIdx(0).hex, idx: 0 }]);
        }}
      >
        作成
      </CreatePollButton>
    </AdminTemplate>
  );
};

const colorOfIdx = (idx: number): color.Color => {
  return [
    color.MainRed,
    color.ChoiceBlue,
    color.ChoiceGreen,
    color.ChoiceYellow,
    color.ChoicePink
  ][idx % 5];
};

const QuestionContainer = styled.div`
  display: flex;
  justify-content: space-between;
  width: 100%;
  margin-bottom: 150px;
  align-items: middle;
`;

const QuestionTag = styled.div`
  width: 150px;
  line-height: 30px;
  font-size: 14px;
  font-weight: 500;
  color: ${color.AdminMainColor.hex};
`;

const QuestionInput = styled.input`
  width: 750px;
  height: 30px;
  padding: 10px 8px;
  margin-right: 69px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
`;

const ChoiceContainer = styled.div`
  width: 100%;
`;

const ChoiceList = styled.div`
  display: flex;
  justify-content: flex-start;
  align-items: middle;
`;

const ChoiceTag = styled.div`
  width: 42px;
  line-height: 30px;
  font-size: 14px;
  font-weight: 500;
  color: ${color.AdminMainColor.hex};
`;

const ChoiceUl = styled.ul`
  margin: 0px;
  list-style: none;
`;

const Choiceitem = styled.li`
  &::before {
    display: inline-block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    margin-right: 20px;
    background-color: ${color.AdminMainColor.hex};
    content: "";
  }
  margin-bottom: 8px;
`;

const ChoiceInput = styled.input`
  width: 150px;
  height: 32px;
  padding: 10px 8px;
  border: solid 1px ${color.AdminInputBorderColor.hex};
  font-size: 10px;
`;

const AddChoice = styled.button`
  display: block;
  width: 100px;
  height: 20px;
  margin: 12px 0 0 69px;
  border-radius: 8px;
  background-color: ${color.WhiteBaseColor.hex};
  border: solid 1px ${color.AdminMainColor.hex};
  font-size: 10px;
  color: ${color.AdminMainColor.hex};
`;

const RemoveChoice = styled.button`
  display: block;
  width: 100px;
  height: 20px;
  margin: 12px 0 0 69px;
  border-radius: 8px;
  background-color: ${color.WhiteBaseColor.hex};
  border: solid 1px ${color.MainRed.hex};
  font-size: 10px;
  color: ${color.MainRed.hex};
`;

const CreatePollButton = styled.button`
  display: block;
  position: absolute;
  bottom: 31px;
  left: 50%;
  transform: translateY(0%) translateX(-50%);
  -webkit-transform: translateY(0%) translateX(-50%);
  width: 250px;
  height: 40px;
  background-color: ${color.MainRed.hex};
  font-size: 14px;
  color: ${color.WhiteBaseColor.hex};
`;

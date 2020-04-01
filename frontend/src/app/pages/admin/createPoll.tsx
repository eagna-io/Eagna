import React from "react";
import styled from "styled-components";
import * as D from "@mojotech/json-type-validation";

import {
  Color,
  AdminBackgroundColor,
  AdminMainColor,
  WhiteBaseColor,
  BlackColor,
  AdminInputBorderColor,
  MainRed,
  ChoiceBlue,
  ChoiceGreen,
  ChoiceYellow,
  ChoicePink
} from "app/components/color";
import * as http from "infra/http";

import { NavigationBar } from "./components/organisms/navbar";

export const CreatePoll: React.FC = () => {
  const [title, setTitle] = React.useState("");
  const [choices, setChoices] = React.useState<[string, string][]>([
    ["", colorOfIdx(0).hex]
  ]);

  return (
    <Container>
      <NavBarComponent>
        <NavigationBar />
      </NavBarComponent>
      <Content>
        <QuestionContainer>
          <QuestionTag>問題文</QuestionTag>
          <QuestionInput
            onChange={e => {
              setTitle(e.target.value);
            }}
          />
        </QuestionContainer>
        <ChoiceContainer>
          <ChoiceList>
            <ChoiceTag>選択肢</ChoiceTag>
            <ChoiceUl>
              {choices.map(([choice, color], i) => (
                <Choiceitem key={i}>
                  <ChoiceInput
                    placeholder="選択肢名"
                    onChange={e => {
                      const newChoices = [...choices];
                      newChoices[i][0] = e.target.value;
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
              setChoices([...choices, ["", nextColor.hex]]);
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
            const res = await http.post({
              path: "/contest/poll",
              body: {
                title,
                choices: Object.fromEntries(choices)
              },
              decoder: D.anyJson()
            });
            alert(JSON.stringify(res));
          }}
        >
          作成
        </CreatePollButton>
      </Content>
    </Container>
  );
};

const colorOfIdx = (idx: number): Color => {
  return [MainRed, ChoiceBlue, ChoiceGreen, ChoiceYellow, ChoicePink][idx];
};

const Container = styled.div`
  width: 100vw;
  height: 100vh;
  background-color: ${AdminBackgroundColor.hex};
  user-select: none;
  display: flex;
`;

const NavBarComponent = styled.div`
  width: 250px;
  height: 100vh;
  background-color: ${AdminMainColor.hex};
  padding-top: 30px;
`;

const Content = styled.div`
  width: 1142px;
  height: calc(100vh - 40px);
  margin: 20px 24px;
  background-color: ${WhiteBaseColor.hex};
  box-shadow: 0 1px 4px 0 ${BlackColor.rgba(0.5)};
  padding: 133px 127px;
  position: relative;
`;

const QuestionContainer = styled.div`
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: middle;
  margin-bottom: 150px;
`;

const QuestionTag = styled.div`
  width: 69px;
  font-size: 14px;
  font-weight: 500;
  line-height: 30px;
  color: ${AdminMainColor.hex};
`;

const QuestionInput = styled.input`
  width: 750px;
  height: 30px;
  padding: 10px 8px;
  font-size: 10px;
  margin-right: 69px;
  border: solid 1px ${AdminInputBorderColor.hex};
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
  font-size: 14px;
  font-weight: 500;
  line-height: 30px;
  color: ${AdminMainColor.hex};
`;

const ChoiceUl = styled.ul`
  margin: 0px;
  list-style: none;
`;

const Choiceitem = styled.li`
  &::before {
    background-color: ${AdminMainColor.hex};
    content: "";
    width: 8px;
    height: 8px;
    display: inline-block;
    border-radius: 50%;
    margin-right: 20px;
  }
  margin-bottom: 8px;
`;

const ChoiceInput = styled.input`
  width: 150px;
  height: 32px;
  padding: 10px 8px;
  font-size: 10px;
  border: solid 1px ${AdminInputBorderColor.hex};
`;

const AddChoice = styled.button`
  width: 100px;
  height: 20px;
  border-radius: 8px;
  background-color: ${WhiteBaseColor.hex};
  border: solid 1px ${AdminMainColor.hex};
  display: block;
  margin: 12px 0 0 69px;
  font-size: 10px;
  color: ${AdminMainColor.hex};
`;

const RemoveChoice = styled.button`
  width: 100px;
  height: 20px;
  border-radius: 8px;
  background-color: ${WhiteBaseColor.hex};
  border: solid 1px ${MainRed.hex};
  display: block;
  margin: 12px 0 0 69px;
  font-size: 10px;
  color: ${MainRed.hex};
`;

const CreatePollButton = styled.button`
  width: 250px;
  height: 40px;
  background-color: ${MainRed.hex};
  display: block;
  font-size: 14px;
  color: ${WhiteBaseColor.hex};
  position: absolute;
  bottom: 31px;
  left: 50%;
  transform: translateY(0%) translateX(-50%);
  -webkit-transform: translateY(0%) translateX(-50%);
`;

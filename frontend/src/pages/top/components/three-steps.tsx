import React, {FC} from 'react';
import styled from 'styled-components';

import {pc} from 'components/responsive';

const ThreeStepsSection: FC = () => {
  return (
    <Container>
      <Head>Eagna in 3 Steps!</Head>
      <StepContainer>
        <StepTitle>
          <StepNum>1</StepNum>. 「参加する」ボタンから市場に参加！
        </StepTitle>
        <StepIcon src="/img/top/step1-icon.png" />
        <StepDesc>
          参加ボタンを押すとコインがもらえます。もらったコインでトークンを売買することができます。
        </StepDesc>
      </StepContainer>
      <StepContainer>
        <StepTitle>
          <StepNum>2</StepNum>. 自分の予想通りにトークンを売買！
        </StepTitle>
        <StepIcon src="/img/top/step2-icon.png" />
        <StepDesc>
          当たったトークンは1つにつき1,000コインに変換されます。価格が安いと思ったときに買い、高いと思ったときに売り、コインを増やしていきましょう！
        </StepDesc>
      </StepContainer>
      <StepContainer>
        <StepTitle>
          <StepNum>3</StepNum>. 正しい予測で報酬をGET！
        </StepTitle>
        <StepIcon src="/img/top/step3-icon.png" />
        <StepDesc>
          コインはいろんな商品やポイントと交換することができます。正確な予測でたくさんの報酬をGETしましょう！
        </StepDesc>
      </StepContainer>
      <TradeButton>今すぐトレード！</TradeButton>
    </Container>
  );
};

export default ThreeStepsSection;

const Container = styled.section`
  width: 90%;
  max-width: 420px;
  margin: 0 auto;
  padding: 50px 0px;
`;

const Head = styled.h3`
  width: 100%;
  margin: 0;
  text-align: center;
  font-size: 20px;
  font-weight: bold;

  ${pc(`
    font-size: 35px;
  `)}
`;

const StepContainer = styled.div`
  width: 100%;
  margin-top: 50px;
`;

const StepTitle = styled.h4`
  width: 100%;
  margin: 0;
  font-size: 14px;
  color: #1c384e;

  ${pc(`
    font-size: 22px;
  `)}
`;

const StepNum = styled.span`
  font-size: 20px;
  color: #1c384e;

  ${pc(`
    font-size: 35px;
  `)}
`;

const StepIcon = styled.img`
  display: inline-block;
  width: 90px;
  height: 90px;
  margin-top: 30px;
  margin-left: 20px;

  ${pc(`
    width: 120px;
    height: 120px;
    margin-top: 40px;
    margin-left: 30px;
  `)}
`;

const StepDesc = styled.p`
  display: inline-block;
  width: calc(100% - 20px - 90px - 35px);
  margin-top: 40px;
  margin-left: 35px;
  vertical-align: top;
  font-size: 10px;
  font-weight: bold;
  line-height: 17px;

  ${pc(`
    width: calc(100% - 30px - 120px - 35px);
    margin-top: 50px;
    font-size: 14px;
  line-height: 24px;
  `)}
`;

const TradeButton = styled.button`
  display: block;
  width: 172px;
  height: 43px;
  margin: 0 auto;
  margin-top: 80px;
  border-radius: 8px;
  border: none;
  background-color: #5bb192;
  box-shadow: 1px 1px 4px 0 rgba(0, 0, 0, 0.5);
  font-size: 15px;
  font-weight: bold;
  color: white;
`;

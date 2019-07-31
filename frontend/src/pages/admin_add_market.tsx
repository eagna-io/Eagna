import React, {FC, useState} from 'react';
import styled from 'styled-components';
import moment from 'moment';

import {postMarket} from 'api/market';
import {User} from 'models/user';
import NotFoundPage from 'pages/not_found';

const EagnaOrganizerId = 'hogehoge';

const AddMarketOrNotFoundPage: FC<{user: User | null}> = ({user}) => {
  if (user !== null && user.isAdmin) {
    return <AddMarketPage user={user} />;
  } else {
    return <NotFoundPage />;
  }
};

export default AddMarketOrNotFoundPage;

const AddMarketPage: FC<{user: User}> = ({user}) => {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [lmsrB, setLmsrB] = useState(100);
  const [openTime, setOpenTime] = useState(moment().format());
  const [closeTime, setCloseTime] = useState(moment().format());
  const [tokens, setTokens] = useState([
    {name: '', description: '', sumbnailUrl: ''},
  ]);
  const [prizes, setPrizes] = useState([
    {name: '', target: '', sumbnailUrl: ''},
  ]);

  const assertNotEmpty = (v: string) => {
    if (v === '') {
      alert('There are empty value!!');
    }
  };

  const assertDateString = (v: string) => {
    if (!moment(v).isValid()) {
      alert('date time format is invalid');
    }
  };

  const validateValues = (): boolean => {
    assertNotEmpty(title);
    assertNotEmpty(description);
    assertDateString(openTime);
    assertDateString(closeTime);
    tokens.forEach(({name, description, sumbnailUrl}) => {
      assertNotEmpty(name);
      assertNotEmpty(description);
      assertNotEmpty(sumbnailUrl);
    });
    prizes.forEach(({name, target, sumbnailUrl}) => {
      assertNotEmpty(name);
      assertNotEmpty(target);
      assertNotEmpty(sumbnailUrl);
    });

    return true;
  };

  const sendRequest = () => {
    user.getAccessToken().then(accessToken => {
      if (accessToken === null) {
        alert('ログインセッションが切れました');
      } else {
        postMarket(
          {
            title: title,
            organizerId: EagnaOrganizerId,
            description: description,
            lmsrB: lmsrB,
            open: moment(openTime),
            close: moment(closeTime),
            tokens: tokens,
            prizes: prizes.map((p, idx) => ({id: idx, ...p})),
          },
          accessToken,
        ).then(marketId => {
          alert(`New market ${marketId} is created`);
        });
      }
    });
  };

  return (
    <Container>
      <h2>Add Market</h2>
      <Field>
        <FieldName>Title :</FieldName>
        <FieldInput
          type="text"
          value={title}
          onChange={e => setTitle(e.target.value)}
        />
      </Field>
      <Field>
        <FieldName>Description :</FieldName>
        <FieldInputTextArea
          value={description}
          onChange={e => setDescription(e.target.value)}
        />
      </Field>
      <Field>
        <FieldName>LMSR β :</FieldName>
        <FieldInput
          type="text"
          value={lmsrB}
          onChange={e => {
            const n = parseInt(e.target.value);
            if (!Number.isNaN(n)) {
              setLmsrB(n);
            }
          }}
        />
      </Field>
      <Field>
        <FieldName>Open time :</FieldName>
        <FieldInput
          type="text"
          value={openTime}
          onChange={e => setOpenTime(e.target.value)}
        />
      </Field>
      <Field>
        <FieldName>Close time :</FieldName>
        <FieldInput
          type="text"
          value={closeTime}
          onChange={e => setCloseTime(e.target.value)}
        />
      </Field>
      <Field>
        <FieldName>Tokens :</FieldName>
        <FieldInputTokens>
          {tokens.map((token, idx) => (
            <div key={idx}>
              <FieldInputTokenName
                type="text"
                value={token.name}
                onChange={e => {
                  const newTokens = tokens.map((t, i) =>
                    i === idx ? {...t, name: e.target.value} : t,
                  );
                  setTokens(newTokens);
                }}
              />
              <FieldInputTokenDesc
                type="text"
                value={token.description}
                onChange={e => {
                  const newTokens = tokens.map((t, i) =>
                    i === idx ? {...t, description: e.target.value} : t,
                  );
                  setTokens(newTokens);
                }}
              />
              <FieldInputTokenSumbnailUrl
                type="text"
                value={token.sumbnailUrl}
                onChange={e => {
                  const newTokens = tokens.map((t, i) =>
                    i === idx ? {...t, sumbnailUrl: e.target.value} : t,
                  );
                  setTokens(newTokens);
                }}
              />
              <FieldRemoveTokenButton
                onClick={() => {
                  const newTokens = tokens.filter((t, i) => i !== idx);
                  setTokens(newTokens);
                }}>
                -
              </FieldRemoveTokenButton>
            </div>
          ))}
          <button
            onClick={() => {
              const newTokens = Array.from(tokens);
              newTokens.push({name: '', description: '', sumbnailUrl: ''});
              console.log(newTokens);
              setTokens(newTokens);
            }}>
            Add token
          </button>
        </FieldInputTokens>
      </Field>
      <Field>
        <FieldName>Prizes :</FieldName>
        <FieldInputPrizes>
          {prizes.map((prize, idx) => (
            <div key={idx}>
              <FieldInputPrizeName
                type="text"
                value={prize.name}
                onChange={e => {
                  const newPrizes = prizes.map((t, i) =>
                    i === idx ? {...t, name: e.target.value} : t,
                  );
                  setPrizes(newPrizes);
                }}
              />
              <FieldInputPrizeTarget
                type="text"
                value={prize.target}
                onChange={e => {
                  const newPrizes = prizes.map((t, i) =>
                    i === idx ? {...t, target: e.target.value} : t,
                  );
                  setPrizes(newPrizes);
                }}
              />
              <FieldInputPrizeSumbnailUrl
                type="text"
                value={prize.sumbnailUrl}
                onChange={e => {
                  const newPrizes = prizes.map((t, i) =>
                    i === idx ? {...t, sumbnailUrl: e.target.value} : t,
                  );
                  setPrizes(newPrizes);
                }}
              />
              <FieldRemovePrizeButton
                onClick={() => {
                  const newPrizes = prizes.filter((t, i) => i !== idx);
                  setPrizes(newPrizes);
                }}>
                -
              </FieldRemovePrizeButton>
            </div>
          ))}
          <button
            onClick={() => {
              const newPrizes = Array.from(prizes);
              newPrizes.push({name: '', target: '', sumbnailUrl: ''});
              console.log(newPrizes);
              setPrizes(newPrizes);
            }}>
            Add prize
          </button>
        </FieldInputPrizes>
      </Field>
      <SubmitButton onClick={() => validateValues() && sendRequest()}>
        Submit
      </SubmitButton>
    </Container>
  );
};

const Container = styled.div`
  width: 980px;
  margin: 0 auto;
  padding: 30px;
`;

const Field = styled.div`
  margin-top: 50px;
`;

const FieldName = styled.div`
  display: inline-block;
  width: 200px;
  margin-right: 50px;
`;

const FieldInput = styled.input`
  width: 670px;
`;

const FieldInputTextArea = styled.textarea`
  width: 670px;
  height: 200px;
  vertical-align: top;
`;

const FieldInputTokens = styled.div`
  display: inline-block;
  width: 670px;
  vertical-align: top;
`;

const FieldInputTokenName = styled.input`
  width: 150px;
  margin-right: 10px;
`;

const FieldInputTokenDesc = styled.input`
  width: 300px;
  margin-right: 10px;
`;

const FieldInputTokenSumbnailUrl = styled.input`
  width: 150px;
  margin-right: 20px;
`;

const FieldRemoveTokenButton = styled.button`
  width: 20px;
  text-align: center;
`;

const FieldInputPrizes = styled.div`
  display: inline-block;
  width: 670px;
  vertical-align: top;
`;

const FieldInputPrizeName = styled.input`
  width: 200px;
  margin-right: 10px;
`;

const FieldInputPrizeTarget = styled.input`
  width: 200px;
  margin-right: 10px;
`;

const FieldInputPrizeSumbnailUrl = styled.input`
  width: 200px;
  margin-right: 20px;
`;

const FieldRemovePrizeButton = styled.button`
  width: 20px;
  text-align: center;
`;

const SubmitButton = styled.button`
  display: block;
  width: 100px;
  height: 50px;
  margin: 50px auto;
  text-align: center;
`;

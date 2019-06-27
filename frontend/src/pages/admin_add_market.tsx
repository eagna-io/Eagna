import React, {FC, useState} from 'react';
import styled from 'styled-components';
import moment from 'moment';

import {postMarket} from 'api/market';
import User from 'models/user';
import NotFoundPage from 'pages/not_found';

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
  const [organizer, setOrganizer] = useState('');
  const [shortDesc, setShortDesc] = useState('');
  const [description, setDescription] = useState('');
  const [lmsrB, setLmsrB] = useState(100);
  const [openTime, setOpenTime] = useState(moment().format());
  const [closeTime, setCloseTime] = useState(moment().format());
  const [tokens, setTokens] = useState([{name: '', description: ''}]);

  const assertNotEmpty = (v: string) => {
    if (v === '') {
      alert('There are empty value!!');
      throw 'assertNotEmpty failed';
    }
  };

  const assertDateString = (v: string) => {
    if (!moment(v).isValid()) {
      alert('date time format is invalid');
      throw 'assertDateString failed';
    }
  };

  const validateValues = (): boolean => {
    assertNotEmpty(title);
    assertNotEmpty(organizer);
    assertNotEmpty(shortDesc);
    assertNotEmpty(description);
    assertDateString(openTime);
    assertDateString(closeTime);
    tokens.forEach(({name, description}) => {
      assertNotEmpty(name);
      assertNotEmpty(description);
    });

    return true;
  };

  const sendRequest = () => {
    postMarket({
      market: {
        title: title,
        organizer: organizer,
        shortDesc: shortDesc,
        description: description,
        lmsrB: lmsrB,
        openTime: moment(openTime),
        closeTime: moment(closeTime),
        tokens: tokens,
      },
      accessToken: user.accessToken,
    }).then(marketId => {
      alert(`New market ${marketId} is created`);
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
        <FieldName>Organization :</FieldName>
        <FieldInput
          type="text"
          value={organizer}
          onChange={e => setOrganizer(e.target.value)}
        />
      </Field>
      <Field>
        <FieldName>Short desc :</FieldName>
        <FieldInput
          type="text"
          value={shortDesc}
          onChange={e => setShortDesc(e.target.value)}
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
              newTokens.push({name: '', description: ''});
              console.log(newTokens);
              setTokens(newTokens);
            }}>
            Add token
          </button>
        </FieldInputTokens>
        <SubmitButton onClick={() => validateValues() && sendRequest()}>
          Submit
        </SubmitButton>
      </Field>
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
  width: 200px;
  margin-right: 10px;
`;

const FieldInputTokenDesc = styled.input`
  width: 400px;
  margin-right: 20px;
`;

const FieldRemoveTokenButton = styled.button`
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

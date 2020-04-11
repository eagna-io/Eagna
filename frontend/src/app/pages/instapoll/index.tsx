import React from "react";
import moment from "moment";

import * as websocket from "infra/ws/contest";

import { Page, LoadingPage } from "./page";
import { reducer, initialState } from "./reducer";

export const InstapollPage: React.FC = () => {
  const [account, setAccount] = React.useState("");
  const [ws, setWs] = React.useState<WebSocket | undefined >();
  const [state, dispatch] = React.useReducer(reducer, initialState);
  const { poll, comments, timer } = state;

  React.useEffect(() => {
    if (account === "") {
      const accountName =
        window.prompt("ユーザー名を入力してください") || "HOGEO";
      setAccount(accountName);
    }
  }, [account]);

  // Websocketコネクションを確立する
  React.useEffect(() => {
    const ws = websocket.open({
      onComment: comment => {
        dispatch({ type: "pushComment", comment });
      },
      onPoll: poll => {
        dispatch({ type: "updatePoll", poll });
      }
    });
    setWs(ws);
  }, []);

  // 一定間隔でtickアクションを送る
  React.useEffect(() => {
    const timer = setInterval(() => {
      dispatch({ type: "tick", time: moment() });
    }, 950);

    return () => {
      clearInterval(timer);
    };
  }, []);
  const testaccount = "test-account";
  const testpoll = {
    id: 'sssss',
    title: 'title',
    endAt: moment(),
    status: "open" as const,
    choices: {
      Lebron: "#424242",
      BBebron: "#424332"
    }
  };
  const testcomments = [{
    account: "test-account",
    comment: "testtest",
    color: "#424242"
  }];
  const testtimer = 300;
  
  // if (poll !== undefined && timer !== undefined && ws !== undefined) {
    return (
      // <Page
      //   account={account}
      //   poll={poll}
      //   comments={comments}
      //   timer={timer}
      //   ws={ws}
      // />

      <Page
        account={testaccount}
        poll={testpoll}
        comments={testcomments}
        timer={testtimer}
      />
    );
  // } else {
  //   return <LoadingPage />;
  // }
};

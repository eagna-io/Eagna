import React from "react";
import moment from "moment";

import * as websocket from "infra/ws/contest";

import { Page, LoadingPage } from "./page";
import { reducer, initialState } from "./reducer";

export const InstapollPage: React.FC = () => {
  const [account, setAccount] = React.useState("");
  const [ws, setWs] = React.useState<WebSocket | undefined>();
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

const testaccount = "test-account";
const testpoll = {
  id: 'sssss',
  idx: 1,
  title: '次にポイントを決めるのは誰？',
  endAt: moment(),
  status: "open" as const,
  choices: {
    Lebron: "#4583e4",
    Lebron青年期: "#4583e4",
    Lebron完全体: "#4583e4",
    KobeBeanBrsssssssssssssssssssssssssssssssssyant: "#e46345",
  }
};
const testcomments = [
  {
    account: "Yuya_F",
    comment: "いけえええええええええええええええええええええええええええええええええええええ!!!!",
    color: "#4583e4"
  },
  {
    account: "Atsuki",
    comment: "いや、いくだろこれは",
    color: "#4583e4"
  },
  {
    account: "ふな",
    comment: "Lebron風引いてるらしいぞ",
    color: "#e46345"
  },
  {
    account: "Yuya_F",
    comment: "嘘やろ",
    color: "#4583e4"
  },
  {
    account: "Atsuki",
    comment: "おわた",
    color: "#c9c8c8"
  },
  {
    account: "Atsuki",
    comment: "やってくれんだろ",
    color: "#4583e4"
  },
  {
    account: "Yuya_F",
    comment: "変えよう",
    color: "#e46345"
  }
];
const testtimer = 300;
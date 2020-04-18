import React from "react";
import moment from "moment";

import * as websocket from "infra/ws/contest";
import * as accountApi from "infra/http/account";
import * as storage from "infra/storage";

import { Page, LoadingPage } from "./page";
import { reducer, initialState } from "./reducer";

interface Props {
  contestId: string;
}

export const InstapollPage: React.FC<Props> = ({ contestId }) => {
  const [accessToken, setAccessToken] = React.useState(storage.getAccessToken);
  const [ws, setWs] = React.useState<WebSocket | undefined>();
  const [state, dispatch] = React.useReducer(reducer, initialState);
  const { poll, comments, timer } = state;

  React.useEffect(() => {
    if (!accessToken) {
      const accountName =
        window.prompt("ユーザー名を入力してください") || "Anonymous";
      accountApi.post(accountName).then(res => {
        storage.setAccessToken(res.access_token);
        setAccessToken(res.access_token);
      });
    }
  }, [accessToken]);

  // Websocketコネクションを確立する
  React.useEffect(() => {
    if (accessToken) {
      const ws = websocket.open({
        contestId,
        accessToken,
        onComment: comment => {
          dispatch({ type: "pushComment", comment });
        },
        onPoll: poll => {
          dispatch({ type: "updatePoll", poll });
        },
        onClosed: closed => {
          console.log(closed);
        }
      });
      setWs(ws);
    }
  }, [accessToken]);

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
      contest={"closed"}
    />
  );
  // } else {
  //   return <LoadingPage />;
  // }
};

const testaccount = "test-account";
const testpoll = {
  id: "sssss",
  idx: 1,
  title: "次にポイントを決めるのは誰？",
  created_at: moment(),
  duration_sec: 30,
  status: "Open" as const,
  choices: [
    { name: "Lebron", color: "#4583e4", idx: 0 },
    { name: "Lebron青年期", color: "#4583e4", idx: 1 },
    { name: "Lebron完全体", color: "#4583e4", idx: 2 },
    {
      name: "KobeBeanBrsssssssssssssssssssssssssssssssssyant",
      color: "#e46345",
      idx: 3
    }
  ],
  // resolved: "Lebron青年期",
  stats: {
    totalVotes: 30,
    votePerChoice: {
      Lebron: 16,
      Lebron青年期: 6,
      Lebron完全体: 4,
      KobeBeanBrsssssssssssssssssssssssssssssssssyant: 4
    }
  }
  // selected: "Lebron青年期"
};
const testcomments = [
  {
    account_name: "Yuya_FYuya_FYuya_F",
    comment:
      "いけえええええええええええええええええええええええええええええええええええええいけええ!!!!",
    color: "#4583e4"
  },
  {
    account_name: "Atsuki",
    comment: "いや、いくだろこれは",
    color: "#4583e4"
  },
  {
    account_name: "ふな",
    comment: "Lebron風引いてるらしいぞ",
    color: "#e46345"
  },
  {
    account_name: "Yuya_F",
    comment: "嘘やろ",
    color: "#4583e4"
  },
  {
    account_name: "AtsukiAtsukiAtsukiAtsukiAtsukiAtsukiAtsuki",
    comment: "おわたおわたおわたおわた",
    color: "#c9c8c8"
  },
  {
    account_name: "Atsuki",
    comment: "やってくれんだろ",
    color: "#4583e4"
  },
  {
    account_name: "Yuya_F",
    comment: "変えよう",
    color: "#e46345"
  }
];
const testtimer = "closed";

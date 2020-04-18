import produce from "immer";
import { Moment } from "moment";

import { Comment, Poll, Timer } from "model/poll";

export type State = {
  poll?: Poll;
  comments: (Comment & { color: string })[];
  timer?: Timer;
};

export const initialState: State = {
  comments: []
};

/*
 * ===========
 * Action
 * ===========
 */
export type Action =
  | {
      type: "tick";
      time: Moment;
    }
  | {
      type: "updatePoll";
      poll: Poll;
    }
  | {
      type: "pushComment";
      comment: Comment;
    };

/*
 * =============
 * Reducer
 * =============
 */
export const reducer = (state: State, action: Action): State => {
  // console.log("======== State =======");
  // console.dir(state);
  // console.log("======== Action =======");
  // console.dir(action);
  return produce(state, state => {
    switch (action.type) {
      case "tick":
        if (state.poll) {
          state.timer =
            state.poll.created_at.unix() +
            state.poll.duration_sec -
            action.time.unix();
          if (state.timer < 0) {
            state.timer = "closed";
          }
        }
        break;
      case "updatePoll":
        if (state.poll?.id !== action.poll.id) {
          // 新しいPollのときはCommentをクリアする
          state.comments = [];
        }
        state.poll = action.poll;
        break;
      case "pushComment":
        const color =
          state.poll?.choices.find(({ name }) => name === action.comment.choice)
            ?.color || "#888888";
        const comment = { ...action.comment, color };
        state.comments = [comment, ...state.comments];
        break;
    }
  });
};

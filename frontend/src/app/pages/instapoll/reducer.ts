import produce from "immer";
import { Moment } from "moment";

import { Comment, Poll, Timer } from "./models";

export type State = {
  poll?: Poll;
  comments: Comment[];
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
export const reducer = (state: State, action: Action): State =>
  produce(state, state => {
    switch (action.type) {
      case "tick":
        if (state.poll && typeof state.timer === "number") {
          state.timer = action.time.unix() - state.poll.endAt.unix();
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
        state.comments.push(action.comment);
        break;
    }
  });

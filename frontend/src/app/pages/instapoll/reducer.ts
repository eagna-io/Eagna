import produce from "immer";

import { Comment, Poll } from "./models";

export type State = {
  poll?: Poll;
  comments: Comment[];
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
      case "updatePoll":
        if (state.poll?.id !== action.poll.id) {
          // 新しいPollのときはCommentをクリアする
          state.comments = [];
        }
        state.poll = action.poll;
        break;
      case "pushComment":
        state.comments.push(action.comment);
    }
  });

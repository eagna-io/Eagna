/*
 * State
 */
export type State = {
  title: string;
  feeds: FeedItem[];
};

export const initialState = {
  title: "",
  feeds: []
};

export type Outcome = "realize" | "unrealize";

export interface FeedItem {
  outcome: Outcome;
  userName: string;
}

/*
 * Action
 */
export type Action =
  | {
      type: "clear";
    }
  | {
      type: "setMarketInfo";
      title: string;
    }
  | {
      type: "addFeedItem";
      outcome: Outcome;
      userName: string;
    };

/*
 * Reducer
 */
export const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    case "clear":
      return { ...initialState };
    case "setMarketInfo":
      return {
        ...state,
        ...{ title: action.title }
      };
    case "addFeedItem":
      const { outcome, userName } = action;
      const clonedFeeds =
        state.feeds.length > 20 ? state.feeds.slice(1) : state.feeds.slice(0);
      clonedFeeds.push({ outcome, userName });
      return { ...state, ...{ feeds: clonedFeeds } };
  }
};

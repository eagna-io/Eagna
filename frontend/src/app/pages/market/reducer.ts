/*
 * State
 */
export type State = {
  title: string;
  outcomes: Outcome[];
  feeds: FeedItem[];
};

export const initialState = {
  title: "",
  outcomes: [],
  feeds: [],
};

export interface Outcome {
  id: string;
  name: string;
}

export interface FeedItem {
  outcomeName: string;
  userName: string;
}

/*
 * Action
 */
export type Action =
  | {
      type: "setMarketInfo";
      title: string;
      outcomes: Outcome[];
    }
  | {
      type: "addFeedItem";
      outcomeId: string;
      userName: string;
    };

/*
 * Reducer
 */
export const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    case "setMarketInfo":
      return {
        ...state,
        ...{ title: action.title, outcomes: action.outcomes }
      };
    case "addFeedItem":
      const { outcomeId, userName } = action;
      const outcomeName = state.outcomes.find(o => o.id === outcomeId)?.name;
      if (!outcomeName) {
        return state;
      }
      const clonedFeeds =
        state.feeds.length > 20
          ? state.feeds.slice(1)
          : state.feeds.slice(0);
      clonedFeeds.push({ outcomeName, userName });
      return { ...state, ...{ feeds: clonedFeeds } };
  }
};

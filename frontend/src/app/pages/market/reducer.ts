import { DateTime } from "model/time";
import { Moment } from "moment";

/*
 * State
 */
export type State = {
  id: string;
  title: string;
  feeds: FeedItem[];
  chartDatas: Data[];
};

export const initialState: State = {
  id: "",
  title: "",
  feeds: [],
  chartDatas: []
};

export type Outcome = "realize" | "unrealize";

export interface FeedItem {
  outcome: Outcome;
  accountName: string;
}

export type Data = [DateTime, number];

/*
 * Action
 */
export type Action =
  | {
      type: "initialize";
      marketId: string;
    }
  | {
      type: "setMarketInfo";
      marketId: string;
      title: string;
    }
  | {
      type: "addOrder";
      marketId: string;
      outcome: Outcome;
      accountName: string;
      time: Moment;
      tipCost: number;
    };

/*
 * Reducer
 */
const MAX_CHART_DUR_MILLIS = 1000 * 60 * 60;

export const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    // MarketのidでStateを初期化する。
    case "initialize":
      return { ...initialState, id: action.marketId };

    // Marketの基本的な情報を更新する(getmarketinfo RPCで取得した情報)
    // 非同期処理の性質上、古いMarketに関する情報更新アクションが
    // 飛んでくることもあるので、そのアクションは無視する。
    case "setMarketInfo":
      if (action.marketId !== state.id) {
        return state;
      } else {
        return {
          ...state,
          title: action.title
        };
      }

    // Feedにitemを追加する。
    // 非同期処理の性質上、古いMarketに関するアクションが
    // 飛んでくることもあるので、そのアクションは無視する。
    case "addOrder":
      if (action.marketId !== state.id) {
        return state;
      } else {
        const { outcome, accountName, time, tipCost } = action;

        // chartDatasに追加
        const needRemoveOldest =
          state.chartDatas[0][0] < time.valueOf() - MAX_CHART_DUR_MILLIS;
        const clonedChartDatas = needRemoveOldest
          ? state.chartDatas.slice(1)
          : state.chartDatas.slice(0);
        clonedChartDatas.push([time.valueOf(), tipCost] as Data);

        // feedsに追加
        const clonedFeeds =
          state.feeds.length > 20 ? state.feeds.slice(1) : state.feeds.slice(0);
        clonedFeeds.push({ outcome, accountName });
        return { ...state, chartDatas: clonedChartDatas, feeds: clonedFeeds };
      }
  }
};

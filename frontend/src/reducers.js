import { combineReducers } from 'redux'
import {
  REQUEST_LOGIN,
  RECEIVE_LOGIN_FAILED,
  RECEIVE_LOGIN_SUCCESS,
  REQUEST_ME,
  RECEIVE_ME_FAILED,
  RECEIVE_ME_SUCCESS,
  REQUEST_MARKET,
  RECEIVE_MARKET_FAILED,
  RECEIVE_MARKET_SUCCESS
} from './actions';


/* --------- Me ---------------- */

const initialMeState = {
  accessToken: null,
  name: null,
  email: null,
  markets: null,
};

function me(state = initialMeState, action) {
  switch (action.type) {
    case REQUEST_LOGIN:
    case RECEIVE_LOGIN_FAILED:
      return initialMeState;
    case RECEIVE_LOGIN_SUCCESS:
      return {
        ...state,
        accessToken: action.payload.accessToken,
      }
      return initialAccountPageState;
    case RECEIVE_ME_FAILED:
      return initialMeState;
    case RECEIVE_ME_SUCCESS:
      return {
        ...state,
        name: action.payload.name,
        email: action.payload.email,
        markets: action.payload.markets,
      };
    default:
      return state;
  }
}


/* --------- Login Page -------- */

const initialLoginPageState = {
  isRequesting: false,
  showFailed: false,
};

function loginPage(state = initialLoginPageState, action) {
  switch (action.type) {
    case REQUEST_LOGIN:
      return {
        accessToken: null,
        isRequesting: true,
        showFailed: false,
      };
    case RECEIVE_LOGIN_FAILED:
      return {
        accessToken: null,
        isRequesting: false,
        showFailed: true,
      }
    case RECEIVE_LOGIN_SUCCESS:
      return {
        accessToken: action.payload.accessToken,
        isRequesting: false,
        showFailed: false,
      }
    default:
      return state;
  }
}


/* --------- Account Page -------- */

const initialAccountPageState = {
  isRequesting: false,
  showError: false,
};

function accountPage(state = initialAccountPageState, action) {
  switch (action.type) {
    case REQUEST_ME:
      return {
        isRequesting: true,
        showError: false,
      }
    case RECEIVE_ME_FAILED:
      return {
        isRequesting: false,
        showError: true,
      }
    case RECEIVE_ME_SUCCESS:
      return {
        isRequesting: false,
        showError: false,
      };
    default:
      return state;
  }
}


/* --------- Market Page -------- */

const initialMarketPageState = {
  isRequesting: false,
  needLogin: false,
  showFailed: false,
  market: null,
};

function marketPage(state = initialMarketPageState, action) {
  switch (action.type) {
    case REQUEST_MARKET:
      return {
        ...state,
        isRequesting: true,
        needLogin: false,
        showFailed: false,
      };
    case RECEIVE_MARKET_FAILED:
      return {
        isRequesting: false,
        needLogin: true,
        showFailed: true,
        market: null,
      };
    case RECEIVE_MARKET_SUCCESS:
      return {
        isRequesting: false,
        showFailed: true,
        market: action.payload,
      };
    default:
      return state;
  }
}


/* ------------- Root ------------- */

export const rootReducer = combineReducers({
  me: me,
  pages: combineReducers({
    login: loginPage,
    account: accountPage,
    market: marketPage,
  })
})

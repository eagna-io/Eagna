import { combineReducers } from 'redux'
import {
  REQUEST_LOGIN,
  RECEIVE_LOGIN_FAILED,
  RECEIVE_LOGIN_SUCCESS,
  REQUEST_ME,
  RECEIVE_ME_FAILED,
  RECEIVE_ME_SUCCESS
} from './actions';


/* --------- My Info -------- */

const initialMeState = {
  accessToken: null,
  name: null,
  coins: null,
  markets: null,
};

function me(state = initialMeState, action) {
  switch (action.type) {
    case REQUEST_LOGIN:
      return {
        ...state,
        accessToken: null,
      };
    case RECEIVE_LOGIN_SUCCESS:
      return {
        ...state,
        accessToken: action.payload.accessToken,
      }
    case RECEIVE_ME_FAILED:
      return {
        ...state,
        accessToken: null,
      };
    case RECEIVE_ME_SUCCESS:
      return {
        ...state,
        name: action.payload.name,
        coins: action.payload.coins,
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
        isRequesting: true,
        showFailed: false,
      };
    case RECEIVE_LOGIN_FAILED:
      return {
        isRequesting: false,
        showFailed: true,
      }
    case RECEIVE_LOGIN_SUCCESS:
      return {
        isRequesting: false,
        showFailed: false,
      }
    default:
      return state;
  }
}


/* --------- Me Page -------- */

const initialMePageState = {
  isRequesting: false,
};

function mePage(state = initialMePageState, action) {
  switch (action.type) {
    case REQUEST_ME:
      return {
        isRequesting: true,
      };
    case RECEIVE_ME_FAILED:
      return {
        isRequesting: false,
      }
    case RECEIVE_ME_SUCCESS:
      return {
        isRequesting: false,
      }
    default:
      return state;
  }
}


/* ------------- Root ------------- */

export const rootReducer = combineReducers({
  me: me,
  pages: combineReducers({
    login: loginPage,
    me: mePage,
  })
})

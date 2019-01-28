import { combineReducers } from 'redux'
import {
  REQUEST_LOGIN,
  RECEIVE_LOGIN_FAILED,
  RECEIVE_LOGIN_SUCCESS,
  REQUEST_ME,
  RECEIVE_ME_FAILED,
  RECEIVE_ME_SUCCESS
} from './actions';

const initialLoginState = {
  isRequesting: false,
  accessToken: undefined,
};

/* { isRequesting : bool
 * , accessToken : string
 * , }
 */
function login(state = initialLoginState, action) {
  switch (action.type) {
    case REQUEST_LOGIN:
      return {
        isRequesting: true,
        accessToken: undefined,
      };
    case RECEIVE_LOGIN_FAILED:
      return {
        isRequesting: false,
        accessToken: undefined,
      }
    case RECEIVE_LOGIN_SUCCESS:
      return {
        isRequesting: false,
        accessToken: action.payload.accessToken,
      }
    default:
      return state;
  }
}

const initialMeState = {
  isRequesting: false,
  name: undefined,
  coins: undefined,
  markets: undefined,
};

function me(state = initialMeState, action) {
  switch (action.type) {
    case REQUEST_ME:
      return {
        ...state,
        isRequesting: true,
      };
    case RECEIVE_ME_FAILED:
      return {
        ...state,
        isRequesting: false,
      };
    case RECEIVE_ME_SUCCESS:
      return {
        ...state,
        isRequesting: false,
        name: action.payload.name,
        coins: action.payload.coins,
        markets: action.payload.markets,
      };
    default:
      return state;
  }
}

export const rootReducer = combineReducers({
  login,
  me
})

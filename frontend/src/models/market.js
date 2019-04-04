import * as lmsr from 'src/lmsr';


export class Market {
  
  // id : int
  // title : string
  // organizer : string
  // shortDesc : string
  // desc : string
  // status : string
  // openTime : Date
  // closeTime : Date
  // tokens : [ Token ]
  // me : {
  //   orderHistry : OrderHistory,
  // }
  constructor(id, title, organizer, shortDesc, desc, status, openTime, closeTime, tokens, me = null, settleToken = null) {
    this.id = id;
    this.title = title;
    this.organizer = organizer;
    this.shortDesc = shortDesc;
    this.desc = desc;
    this.status = status;
    this.openTime = openTime;
    this.closeTime = closeTime;
    this.tokens = tokens;
    this.settleToken = settleToken;
    this.me = me;
  }
}

export class Token {

  // id : int
  // name : string
  // desc : string
  // amount : int
  // price : int
  constructor(id, name, desc, amount, price) {
    this.id = id;
    this.name = name;
    this.desc = desc;
    this.amount = amount;
    this.price = price;
  }

  /*
    # Args
    - distribution : [ {
        "id" : int,
        "name" : string,
        "desc" : string,
        "amount" : int,
      }]

    # Returns
    - [ Token ]
  */ 
  static fromDistribution(distribution) {
    const prices = lmsr.prices(distribution.map(item => item.amount));
    return distribution.map((item, i) => {
      return new Token(item.id, item.name, item.desc, item.amount, prices[i]);
    });
  }
}


export class OrderRecord {

  /*
    # Args
    - id : int
    - token : Token
    - type : string
    - amountToken : int
    - amountCoin : int
    - time : Date
  */
  constructor(id, token, type, amountToken, amountCoin, time) {
    this.id = id;
    this.token = token;
    this.type = type;
    this.amountToken = amountToken;
    this.amountCoin = amountCoin;
    this.time = time;
  }
}


export class OrderHistory {
  constructor(orderRecords) {
    this.records = orderRecords;
  }

  currentAmountToken(token) {
    const reducer = (sum, record) =>
      sum + record.token === token ? record.amountToken : 0;
    return this.records.reduce(reducer, 0);
  }

  currentAmountCoin() {
    return this.records.reduce((sum, record) => sum + record.amountCoin, 0);
  }
}
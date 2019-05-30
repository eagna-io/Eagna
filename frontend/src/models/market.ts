interface Market {
  id: number;
  title: string;
  organizer: string;
  short_desc: string;
  description: string;
  open_time: Date;
  close_time: Date;
  tokens: Token[];
  status: string;
  settle_token_id?: number;
  orders?: Order[];
}

export default Market;

export interface Token {
  id: number;
  name: string;
  description: string;
}

export interface Order {
  token_id: number;
  amount_token: number;
  amount_coin: number;
  time: Date;
}

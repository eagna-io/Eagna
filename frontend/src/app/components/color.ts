export class Color {
  readonly hex: string;

  constructor(
    readonly red: number,
    readonly green: number,
    readonly blue: number
  ) {
    this.hex = `#${toHex(this.red)}${toHex(this.green)}${toHex(this.blue)}`;
  }

  static fromHex(hex: string): Color {
    const red = fromHex(hex.slice(1, 3));
    const green = fromHex(hex.slice(3, 5));
    const blue = fromHex(hex.slice(5, 7));
    return new Color(red, green, blue);
  }
}

function toHex(i: number): string {
  return ("0" + i.toString(16)).slice(-2);
}

function fromHex(hex: string): number {
  return parseInt(hex, 16);
}

export const BackgroundMainColor = Color.fromHex("#242423");

export const PurpleColor = Color.fromHex("#BB86FC");
export const RankingColor = Color.fromHex("#FAD160");
export const TextBaseColor = Color.fromHex("#AEAEAE");
export const RedDisagreeColor = Color.fromHex("#F74C61");
export const GreenAgreeColor = Color.fromHex("#39CCBE");
export const MarketPredictionValueColor = Color.fromHex("#9B9A9A");
export const ItemContainerBgColor = Color.fromHex("#535151");
export const MineColor = Color.fromHex("#F8E71C");

export const EagnaColor = Color.fromHex("#5BB192");
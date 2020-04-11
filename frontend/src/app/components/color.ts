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

  // opacity; 0 ~ 1
  hexWithOpacity(opacity: number): string {
    return `${this.hex}${toHex(opacity * 256)}`;
  }

  rgba(opacity: number): string {
    return `rgba(${this.red}, ${this.green}, ${this.blue}, ${opacity})`;
  }
}

function toHex(i: number): string {
  return ("0" + i.toString(16)).slice(-2);
}

function fromHex(hex: string): number {
  return parseInt(hex, 16);
}

export const BackgroundMainColor = Color.fromHex("#bb86fc");
export const WhiteBaseColor = Color.fromHex("#ffffff");
export const BlackColor = Color.fromHex("#000000");
export const TextBaseColor = Color.fromHex("#333333");
export const MainRed = Color.fromHex("#E46345");

export const ChoiceBlue = Color.fromHex("#4583E4");
export const ChoiceGreen = Color.fromHex("#57E445");
export const ChoiceYellow = Color.fromHex("#E4DC45");
export const ChoicePink = Color.fromHex("#E445B3");

export const WildWatermelon = Color.fromHex("#fd6585");
export const ToreaBay = Color.fromHex("#0d25b9");
export const Alto = Color.fromHex("#d8d8d8");


export const Correct = Color.fromHex("#18e68c");

export const ShadowGray = Color.fromHex("#616161");
export const VoteRateBackGround = Color.fromHex("#C9C8C8");

export const AdminBackgroundColor = Color.fromHex("#F4FAFF");
export const AdminMainColor = Color.fromHex("#1b384e");
export const AdminInputBorderColor = Color.fromHex("#d9d9d9");

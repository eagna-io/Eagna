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

export const NavyBlue = Color.fromHex("#1C384E");

export const UpcomingMarketColor = Color.fromHex("#D8D212");
export const OpenMarketColor = Color.fromHex("#23AC0E");
export const ClosedMarketColor = Color.fromHex("#3261AB");
export const ResolvedMarketColor = Color.fromHex("#A52175");

export const EagnaColor = Color.fromHex("#5BB192");

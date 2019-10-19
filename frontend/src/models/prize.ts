import { Moment } from "moment";

import { EagnaPrizeApi } from "infra/eagna/prize";

export class Prize {
  constructor(
    readonly id: string,
    readonly name: string,
    readonly description: string,
    readonly thumbnailUrl: string,
    readonly point: number,
    readonly available: boolean,
    readonly created: Moment
  ) {}
}

export class PrizeRepository {
  static async queryAll(): Promise<Prize[]> {
    const prizeList = await EagnaPrizeApi.queryAll();
    return prizeList.map(
      prize =>
        new Prize(
          prize.id,
          prize.name,
          prize.description,
          prize.thumbnailUrl,
          prize.point,
          prize.available,
          prize.created
        )
    );
  }
}

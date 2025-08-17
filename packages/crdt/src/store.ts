import { Rga, RgaUnit, type ActorId, type RgaUnitId } from ".";

export class RgaDeserializer {
  public static from_array<T>(array: RgaStoreUnit<T>[], actorId: ActorId, start: T): Rga<T> {
    const rga = new Rga(actorId, start);

    for (const unit of array) {
      const rgaUnit = new RgaUnit(unit.id, unit.contents);
      rgaUnit.isTombstone = unit.is_tombstone;
      rga.insertLast(rgaUnit);
    }

    return rga;
  }
}

export class RgaStoreUnit<T> {
  constructor(public id: RgaUnitId,
    public contents: T,
    public is_tombstone: boolean) { }
}

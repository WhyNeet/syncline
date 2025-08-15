export type ActorId = number;
export type ActorClock = number;

export class Rga<T> {
  private _clock: ActorClock;
  private _actorId: ActorId;
  private _root: RgaUnit<T>;

  constructor(actorId: ActorId, start: T) {
    this._actorId = actorId;
    this._clock = 0;
    this._root = new RgaUnit([0, 0], start);
  }

  public insert(
    insertQuery: RgaInsertQuery,
    contents: T,
    actor_id: ActorId | null,
    id: ActorClock | null,
  ): RgaUnitId | null {
    const actorId = actor_id ?? this._actorId;
    const runQuery = () => {
      if (query.insert.is.right(insertQuery)) {
        const id = insertQuery;
        let unit = this._root;

        while (true) {
          if (idUtil.equal(unit.id, id)) {
            let prev = unit;
            while (true) {
              let next = prev.next;
              if (next === null || actorId < next.id[0]) {
                return prev;
              }

              prev = next;
            }
          }

          if (unit.next !== null) {
            unit = unit.next;
          } else return null;
        }
      }

      const leftId = insertQuery[0];
      const rightId = insertQuery[1];

      let unit = this._root;

      while (true) {
        if (unit.id === leftId) {
          let next = unit.next;
          if (!next) return null;
          if (next.id !== rightId) {
            let prev = unit;

            while (true) {
              let next = prev.next;
              if (!next) return null;

              if (next.id === rightId || actorId < next.id[0]) return prev;
              prev = next;
            }
          }

          return unit;
        }

        if (unit.next !== null) unit = unit.next;
        else return null;
      }
    };

    let prevUnit = runQuery();
    if (!prevUnit) return null;

    const unitId: RgaUnitId = [actorId, id ?? ++this._clock];
    let tmpNext = prevUnit.next;
    let newUnit = new RgaUnit(unitId, contents);
    newUnit.next = tmpNext;
    prevUnit.next = newUnit;

    return unitId;
  }

  public toString(): string {
    let result = "";
    if (this._root.next === null) {
      return result;
    }
    let unit = this._root.next!;

    while (true) {
      if (!unit.isTombstone) result += unit.contents;

      if (unit.next !== null) unit = unit.next;
      else break;
    }

    return result;
  }
}

export type RgaRightInsertQuery = RgaUnitId;
export type RgaMiddleInsertQuery = [RgaUnitId, RgaUnitId];
export type RgaInsertQuery = RgaRightInsertQuery | RgaMiddleInsertQuery;

export const query = {
  insert: {
    is: {
      right: (query: RgaInsertQuery): query is RgaRightInsertQuery =>
        typeof query[0] === "number",
      middle: (query: RgaInsertQuery): query is RgaMiddleInsertQuery =>
        typeof query[0] !== "number",
    },
  },
};
export const idUtil = {
  equal: (a: RgaUnitId, b: RgaUnitId) => a[0] === b[0] && a[1] === b[1],
};

export type RgaUnitId = [ActorId, ActorClock];

export class RgaUnit<T> {
  private _next: RgaUnit<T> | null;
  private _isTombstone: boolean;
  private _contents: T;
  private _id: RgaUnitId;

  constructor(id: RgaUnitId, contents: T) {
    this._next = null;
    this._isTombstone = false;
    this._contents = contents;
    this._id = id;
  }

  public get id(): RgaUnitId {
    return this._id;
  }

  public get contents(): T {
    return this._contents;
  }

  public get isTombstone(): boolean {
    return this._isTombstone;
  }
  public set isTombstone(value: boolean) {
    this._isTombstone = value || this._isTombstone;
  }

  public get next(): RgaUnit<T> | null {
    return this._next;
  }

  public set next(value: RgaUnit<T> | null) {
    this._next = value;
  }
}

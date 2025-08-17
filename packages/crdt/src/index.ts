export type ActorId = number;
export type ActorClock = number;

export class Rga<T> {
  private _clock: ActorClock;
  private _actorId: ActorId;
  private _root: RgaUnit<T>;
  private _version: VersionVector;

  constructor(actorId: ActorId, start: T) {
    this._actorId = actorId;
    this._clock = 0;
    this._root = new RgaUnit([0, 0], start);
    this._version = new VersionVector();
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
              if (prev.next === null || actorId <= prev.next.id[0]) return prev;

              prev = prev.next;
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
        if (idUtil.equal(unit.id, leftId)) {
          let next = unit.next;
          if (!next) return null;
          if (!idUtil.equal(next.id, rightId)) {
            let prev = unit;

            while (true) {
              let next = prev.next;
              if (!next) return null;

              if (idUtil.equal(next.id, rightId) || actorId < next.id[0])
                return prev;
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

    this._version.nextVersion();

    return unitId;
  }

  public insertLast(unit: RgaUnit<T>) {
    let last = this._root;

    while (last.next !== null) {
      last = last.next;
    }

    last.next = unit;
  }

  public queryAt(index: number): RgaUnit<T> | null {
    let current = 0;
    let unit = this._root;
    while (true) {
      if (current === index) return unit;
      if (unit.next) {
        unit = unit.next;
        if (!unit.isTombstone) current++;
      } else return null;
    }
  }

  public delete(id: RgaUnitId) {
    if (id[1] === 0) return;

    let unit = this._root;

    while (!idUtil.equal(unit.id, id)) {
      if (unit.next !== null) unit = unit.next;
      else return;
    }

    unit.isTombstone = true;

    this._version.nextVersion();
  }

  public compact() {
    let unit = this._root;

    while (unit.next !== null) {
      if (unit.next.isTombstone) {
        let next_next = unit.next.next;
        unit.next = next_next;
      } else {
        unit = unit.next;
      }
    }

    this._version.nextVersion();
    this._version.markCompaction();
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

  public get version(): VersionVector {
    return this._version;
  }
}

export class VersionVector {
  private _version: number;
  private _lastCompaction: number;

  constructor() {
    this._version = 0;
    this._lastCompaction = 0;
  }

  public nextVersion() {
    this._version++;
  }

  public markCompaction() {
    this._lastCompaction = this._version;
  }

  public get lastCompaction(): number {
    return this._lastCompaction;
  }

  public get version(): number {
    return this._version;
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

export * from "./store";

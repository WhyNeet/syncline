import type { ActorId, RgaInsertQuery, RgaStoreUnit, RgaUnitId } from "crdt";

export interface RealtimeEvent {
  kind: RealtimeEventKind,
  version: {
    version: number;
    last_compaction: number;
  },
  actor: ActorId
}

export type RealtimeEventKind = RealtimeInsertEvent | RealtimeDeleteEvent | RealtimeCompactEvent | RealtimeStateSyncEvent;

export interface RealtimeInsertEvent {
  kind: "Insert";
  id: RgaUnitId;
  contents: string;
  query: RgaInsertQuery;
}

export interface RealtimeDeleteEvent {
  kind: "Delete";
  id: RgaUnitId;
}

export interface RealtimeStateSyncEvent {
  kind: "StateSync",
  state: RgaStoreUnit<string>[]
}

export interface RealtimeCompactEvent {
  kind: "Compact";
}

export type IncomingEvent = RealtimeEvent | SystemEvent;
export type SystemEvent = ActorIdEvent;

export interface ActorIdEvent {
  actor_id: number;
}

export const eventUtil = {
  incoming: {
    is: {
      system: (e: IncomingEvent): e is SystemEvent =>
        typeof (e as unknown as Record<string, unknown>)["kind"] ===
        "undefined",
    },
  },
};

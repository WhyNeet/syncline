import type { RgaInsertQuery, RgaUnitId } from "crdt";

export type RealtimeEvent = RealtimeInsertEvent | RealtimeDeleteEvent;

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

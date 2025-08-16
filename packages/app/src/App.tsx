import { Rga, type RgaInsertQuery } from "crdt";
import { createEffect, createSignal, onMount } from "solid-js";
import { eventUtil, type IncomingEvent, type RealtimeEvent } from "./lib/event";

function App() {
  let crdt: Rga<string> | null = null;
  let ws: WebSocket | null = null;
  const [actorId, setActorId] = createSignal<number | null>(null);
  const [contents, setContents] = createSignal("");
  const [selection, setSelection] = createSignal(0);
  let textarea: HTMLTextAreaElement = null!;

  onMount(() => {
    ws = new WebSocket("http://localhost:8080/api/docs/0");
    ws.addEventListener("message", (data) => {
      const event: IncomingEvent = JSON.parse(data.data);
      if (eventUtil.incoming.is.system(event)) {
        crdt = new Rga(event.actor_id, "");
        setActorId(event.actor_id);
      } else if (crdt) {
        switch (event.kind) {
          case "Insert":
            crdt.insert(event.query, event.contents, event.id[0], event.id[1]);
            break;
          case "Delete":
            crdt.delete(event.id);
            break;
        }
        textarea.value = crdt.toString();
      }
    });

    return () => {
      ws?.close();
    };
  });

  function handleKeyUp() {
    setSelection(textarea.selectionStart);
  }

  createEffect(() => {
    textarea.setSelectionRange(selection(), selection());
  });

  function handleInput(
    event: InputEvent & {
      currentTarget: HTMLTextAreaElement;
      target: HTMLTextAreaElement;
    },
  ) {
    event.preventDefault();
    if (!crdt || !ws) return;
    const cursorPosition = event.currentTarget.selectionStart;
    console.log(event.inputType);
    switch (event.inputType) {
      case "insertText": {
        const unit = crdt.queryAt(cursorPosition - 1)!;
        const next = unit.next;
        const query: RgaInsertQuery = next ? [unit.id, next.id] : unit.id;
        const data = event.data!.charAt(0);
        const id = crdt.insert(query, data, null, null);
        const wsEvent: RealtimeEvent = {
          kind: "Insert",
          id: id!,
          contents: event.data!.charAt(0),
          query,
        };
        ws.send(JSON.stringify(wsEvent));
        break;
      }
      case "deleteContentBackward": {
        const unit = crdt.queryAt(cursorPosition + 1)!;
        crdt.delete(unit.id);
        const wsEvent: RealtimeEvent = {
          kind: "Delete",
          id: unit.id
        };
        ws.send(JSON.stringify(wsEvent));
      }
        break;
    }

    setContents(crdt.toString());

    textarea.value = contents();
  }

  return (
    <main class="h-screen w-screen p-10 flex flex-col">
      <h1 class="font-bold text-lg mb-10">Editor</h1>
      {actorId() ? <div>Actor id: {actorId()}</div> : null}
      <textarea
        ref={textarea}
        class="resize-none border border-neutral-200 flex-1 w-full"
        value={contents()}
        onKeyUp={handleKeyUp}
        onInput={handleInput}
        autocomplete="off"
        autocorrect="off"
      ></textarea>
    </main>
  );
}

export default App;

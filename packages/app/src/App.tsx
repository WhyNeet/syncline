import { Rga } from "crdt";
import { createEffect, createSignal } from "solid-js";

function App() {
  const crdt = new Rga(0, "");
  const [contents, setContents] = createSignal("");
  const [selection, setSelection] = createSignal(0);
  let textarea: HTMLTextAreaElement = null!;

  function handleKeyUp() {
    console.log(textarea.selectionStart);
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
    const cursorPosition = event.currentTarget.selectionStart;
    console.log(event.inputType);
    switch (event.inputType) {
      case "insertText": {
        const unit = crdt.queryAt(cursorPosition - 1)!;
        const next = unit.next;
        console.log("\"" + event.data + "\"");
        if (next) crdt.insert([unit.id, next.id], event.data!.charAt(0), null, null);
        else crdt.insert(unit.id, event.data!.charAt(0), null, null);
        break;
      }
      case "deleteContentBackward": {
        const unit = crdt.queryAt(cursorPosition + 1)!;
        crdt.delete(unit.id);
      }
        break;
    }

    setContents(crdt.toString());

    textarea.value = contents();
  }

  return (
    <main class="h-screen w-screen p-10 flex flex-col">
      <h1 class="font-bold text-lg mb-10">Editor</h1>
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

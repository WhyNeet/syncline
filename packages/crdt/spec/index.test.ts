import { test, expect } from "vitest";
import { Rga } from "../src";

test("Rga insertion works", () => {
  const rga = new Rga(0, "");
  const input = "Hello, world!";
  let prevId = rga.insert([0, 0], input.charAt(0), null, null);
  expect(prevId).toStrictEqual([0, 1]);
  for (let i = 1; i < input.length; i++) {
    prevId = rga.insert(prevId!, input.charAt(i), null, null);
    expect(prevId).toStrictEqual([0, i + 1]);
  }

  expect(rga.toString()).toBe("Hello, world!");
});

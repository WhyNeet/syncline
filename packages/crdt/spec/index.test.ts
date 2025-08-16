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

test("Rga editor positioning works", () => {
  const rga = new Rga(0, "");
  const input = "Hello, world!";
  let prevId = rga.insert([0, 0], input.charAt(0), null, null);
  expect(prevId).toStrictEqual([0, 1]);
  for (let i = 1; i < input.length; i++) {
    prevId = rga.insert(prevId!, input.charAt(i), null, null);
    expect(prevId).toStrictEqual([0, i + 1]);
  }

  rga.delete([0, 6]);
  rga.delete([0, 7]);
  expect(rga.insert([[0, 7], [0, 8]], " ", null, null)).toStrictEqual([0, 14]);

  expect(rga.queryAt(6)?.id).toStrictEqual([0, 14]);
})

test("Rga deletion works", () => {
  const rga = new Rga(0, "");
  const input = "Hello, world!";
  let prevId = rga.insert([0, 0], input.charAt(0), null, null);
  expect(prevId).toStrictEqual([0, 1]);
  for (let i = 1; i < input.length; i++) {
    prevId = rga.insert(prevId!, input.charAt(i), null, null);
    expect(prevId).toStrictEqual([0, i + 1]);
  }

  rga.delete([0, 6]);
  rga.delete([0, 7]);
  expect(rga.insert([[0, 7], [0, 8]], " ", null, null)).toStrictEqual([0, 14]);

  expect(rga.toString()).toBe("Hello world!");
});

test("Rga compaction works", () => {
  const rga = new Rga(0, "");
  const input = "Hello, world!";
  let prevId = rga.insert([0, 0], input.charAt(0), null, null);
  expect(prevId).toStrictEqual([0, 1]);
  for (let i = 1; i < input.length; i++) {
    prevId = rga.insert(prevId!, input.charAt(i), null, null);
    expect(prevId).toStrictEqual([0, i + 1]);
  }

  rga.delete([0, 6]);
  rga.delete([0, 7]);
  expect(rga.insert([[0, 7], [0, 8]], " ", null, null)).toStrictEqual([0, 14]);

  rga.compact();

  expect(rga.insert([[0, 6], [0, 7]], " ", null, null)).toBeNull();
});

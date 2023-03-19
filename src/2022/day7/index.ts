// @ts-nocheck
import fs from "fs";
import path from "path";

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8");

const tree = {
  // https://stackoverflow.com/a/18937118/9823455
  set(path: string, value: any) {
    let schema = this; // a moving reference to internal objects within obj
    const pList = path.split(/[/]/i);

    // handle path of arbitrary depth
    const len = pList.length;
    for (let i = 0; i < len - 1; i++) {
      const elem = pList[i];
      schema[elem] ??= {};
      schema = schema[elem];
    }
    schema[pList[len - 1]] = value;
  },
};

function main() {
  const lines = input.split("\n");
  let currPath = [];

  for (let line of lines) {
    const parts = line.split(" ");

    // command
    if (parts[0] === "$") {
      // command cd
      if (parts.length === 3) {
        switch (parts[2]) {
          case "/": // noop
            break;
          case "..":
            currPath.pop();
            break;
          default:
            currPath.push(parts[2]);
            break;
        }
      }
      // command ls
      if (parts.length === 2) {
        // noop
      }
    }
    // dir
    if (parts[0] === "dir") {
      // noop
    }
    // file
    if (!isNaN(parts[0])) {
      // set file size
      const [filesize, key] = parts;
      const value = parseInt(filesize);
      tree.set(currPath.concat(key).join("/"), value);
    }
  }

  const sums = [];

  // depth-first-traversal — push DIRECTORY sizes onto a list
  function getSizes(node: any): number {
    let size = 0;
    Object.entries(node).forEach(([key, value]) => {
      // ignore the `set` function
      if (key === "set") {
        return; // noop
      }
      // return size of file, but do NOT push onto list
      if (typeof value === "number") {
        size += value;
      }
      // recurse and push size of dir onto list
      if (typeof value === "object") {
        let res = getSizes(value);
        sums.push(res);
        size += res;
      }
    });
    return size;
  }

  // Root size = 43636666; This does not get pushed onto the sums list
  const root = getSizes(tree);

  const sum = [...sums]
    .filter((e) => e <= 100_000)
    .reduce((acc: number, next: number) => acc + next, 0);

  // 6_385_359 too high, because I didn't read the directions and summed up file (leaf node) sizes too.
  // 1_297_159 correct, because this only sums up the directory sizes.
  console.log({ sum });

  // part2

  // total available disk space 70_000_000
  // my root size               43_636_666
  // diff                       26_363_334

  // need to delete the smallest dir whose
  // size is AT LEAST 30_000_000 - 26_363_334
  const remainingSpace = 70_000_000 - root;
  console.log(
    Math.min(...[...sums].filter((e) => e >= 30_000_000 - remainingSpace))
  );

  // 3_866_390
}

main();

const IO = (unsafeFunction) => ({
  map: (f) => IO(() => f(unsafeFunction())),
  bind: (f) => IO(() => f(unsafeFunction()).unsafeRun()),
  unsafeRun: unsafeFunction,
});

const log = (x) => IO(() => console.log(x));

const readFile = (path) => IO(() => `${path} -> file contents`);

readFile("filepath")
  .map((str) => str.toUpperCase())
  .bind(log)
  .unsafeRun();

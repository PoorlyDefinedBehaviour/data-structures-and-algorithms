const Task = (f) => ({
  map: (g) => Task((reject, resolve) => f(reject, (x) => resolve(g(x)))),
  bind: (g) =>
    Task((reject, resolve) => f(reject, (x) => g(x).fork(reject, resolve))),
  fork: f,
});

const fetchUserByUsername = (username) =>
  Task((_, resolve) => resolve({ id: 1, username }));

const fechTweetsByUserId = (userId) =>
  Task((_, resolve) =>
    resolve([
      { id: 1, user_id: userId, body: "foo" },
      { id: 2, user_id: userId, body: "bar" },
      { id: 3, user_id: userId, body: "baz" },
    ])
  );

fetchUserByUsername("john_doe_1337")
  .bind((user) => fechTweetsByUserId(user.id))
  .fork(console.error, console.log);

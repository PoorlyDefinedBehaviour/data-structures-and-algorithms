const fst = pair => pair.first
const snd = pair => pair.second

const range = start => end =>
  start < end ? { first: start, second: range(start + 1)(end) } : null

const take = amount => pair => {
  const doTake = count => p =>
    count < amount ? { first: fst(p), second: doTake(count + 1)(snd(p)) } : null

  return doTake(0)(pair)
}

const drop = amount => pair => {
  const doDrop = count => p => (count < amount ? snd(doDrop(count + 1)(p)) : p)

  return doDrop(0)(pair)
}

const map = pair => mapper =>
  pair ? { first: mapper(fst(pair)), second: map(snd(pair))(mapper) } : null

const toArray = pair =>
  pair && snd(pair)
    ? [].concat([fst(pair)]).concat(toArray(snd(pair)))
    : fst(pair)

const fromArray = array => ({
  first: array[0],
  second: array.length > 1 ? fromArray(array.slice(1)) : null
})

console.log(toArray(map(range(0)(5))(x => x * 2)))

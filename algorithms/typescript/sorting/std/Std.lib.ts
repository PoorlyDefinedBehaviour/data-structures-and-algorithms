export const tap = (f: Function) => <T>(x: T) => (f(x), x);

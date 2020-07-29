import Prelude hiding(foldl, head, tail, map, filter, reverse, takeWhile, dropWhile, List)
import Data.Bits

data List a = Cons a (List a) | Nil deriving (Show)

append :: List a -> List a -> List a
append xs Nil = xs
append Nil ys = ys
append (Cons x xs) ys = Cons x (xs `append` ys)

instance Semigroup (List a) where
  -- (<>) :: Semigroup s => s a -> s a -> s a
  (<>) = append

instance Monoid (List a) where
  mempty = Nil

instance Functor List where 
  -- fmap :: Functor f => (a -> b) -> f a -> f b
  fmap f Nil = Nil
  fmap f (Cons x xs) = Cons(f x) (fmap f xs)

instance Applicative List where
  -- pure :: Applicative t => a -> t a
  pure = (flip Cons) Nil
  -- (<*>) :: Applicative t => (a -> b) -> t a -> t b
  Nil <*> _ = Nil
  _ <*> Nil = Nil
  (Cons f fs) <*> xs = (fmap f xs) `append` (fs <*> xs)

head :: List a -> a
head (Cons x _) = x

tail :: List a -> List a
tail (Cons _ xs) = xs

foldl :: (b -> a -> b) -> b -> List a -> b
foldl f x Nil = x
foldl f x ys = foldl f (f x (head ys)) (tail ys)

map :: (a -> b) -> List a -> List b
map _ Nil = Nil
map f (Cons x xs) = Cons (f x) (map f xs)

filter :: (a -> Bool) -> List a -> List a
filter _ Nil = Nil
filter p (Cons x xs) = if p x then Cons x (filter p xs) else (filter p xs)

reverse :: List a -> List a
reverse Nil = Nil
reverse (Cons x xs) = (reverse xs) <> (Cons x Nil)

takeWhile :: (a -> Bool) -> List a -> List a
takeWhile _ Nil = Nil
takeWhile p (Cons x xs) = if p x then (Cons x Nil) <> (takeWhile p xs) else Nil

dropWhile :: (a -> Bool) -> List a -> List a
dropWhile _ Nil = Nil
dropWhile p (Cons x xs) = if p x then (dropWhile p xs) else Cons x xs

isEven :: Int -> Bool
isEven x = (x .&. 1) == 0

main :: IO ()
main = do
  let list = Cons 1 (Cons 2 (Cons 3 Nil))

  print (map (\x -> x * 2) list)
  print (filter isEven list)
  print (reverse list)
  print (takeWhile isEven list)
  print (dropWhile isEven list)


data BinaryTree a = Leaf a | Node (BinaryTree a) (BinaryTree a) deriving (Show)

sumLeafs :: BinaryTree Int -> Int
sumLeafs (Leaf x) = x
sumLeafs (Node left right) = sumLeafs left + sumLeafs right

main :: IO ()
main = do
  print
    ( sumLeafs
        ( Node
            (Node (Leaf 1) (Leaf 2))
            (Node (Leaf 4) (Leaf 5))
        )
    )
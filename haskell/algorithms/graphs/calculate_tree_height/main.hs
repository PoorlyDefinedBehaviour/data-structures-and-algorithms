data BinaryTree a = Leaf a | Node (BinaryTree a) (BinaryTree a) deriving (Show)

height :: BinaryTree Int -> Int
height (Leaf _) = 1
height (Node left right) = max (height left) (height right) + 1

main :: IO ()
main = do
  --        o
  --      /   \
  --    o       o
  --   /  \   /  \
  --  1   2  4    5

  print
    ( height
        ( Node
            (Node (Leaf 1) (Leaf 2))
            (Node (Leaf 4) (Leaf 5))
        )
    )
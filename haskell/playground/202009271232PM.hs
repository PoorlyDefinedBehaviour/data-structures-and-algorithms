data BinaryTree a = Leaf | Node a (BinaryTree a) (BinaryTree a) deriving (Show)

binaryTreeFromList :: Ord a => [a] -> BinaryTree a
binaryTreeFromList [] = Leaf
binaryTreeFromList (x : xs) =
  Node
    x
    (binaryTreeFromList (filter (< x) xs))
    (binaryTreeFromList (filter (> x) xs))

main :: IO ()
main = do
  print $ binaryTreeFromList [1, 2, 3]
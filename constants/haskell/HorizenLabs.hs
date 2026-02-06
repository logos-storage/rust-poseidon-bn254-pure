
--
-- There are two sets of instances, because very unfortunately, HorizenLabs
-- actually changed their constants in the commit:
--
-- <https://github.com/HorizenLabs/poseidon2/commit/bb476b9ca38198cf5092487283c8b8c5d4317c4e>
-- 
-- Which is obviously very bad for cross-project compatibility...
--
-- Anyway, the instance constants can be found in
--
-- * HorizenLabs/Old.hs
-- * HorizenLabs/New.hs
--

module HorizenLabs where

--------------------------------------------------------------------------------

import Data.List ( transpose )
import Control.Monad
import System.Random

--------------------------------------------------------------------------------
-- external matrices (from the paper)

matrixMulTest :: Num a => [[Integer]] -> [a] -> [a]
matrixMulTest matrix vector
  | length matrix /= n                = error "matrixMul: wrong vertical dimension"
  | any (\v -> length v /= n ) matrix = error "matrixMul: wrong horizontal dimension"
  | otherwise = [ sum (zipWith f column vector) | column <- matrix ]
  where
    n = length vector
    f coeff x = fromInteger coeff * x

mds_matrix_M2_row_major :: [[Integer]]
mds_matrix_M2_row_major =
  [ [ 2 , 1 ]
  , [ 1 , 2 ]
  ]

mds_matrix_M3_row_major :: [[Integer]]
mds_matrix_M3_row_major =
  [ [ 2 , 1 , 1 ]
  , [ 1 , 2 , 1 ]
  , [ 1 , 1 , 2 ] 
  ]

----------------------------------------

--
-- the multiplication by this matrix 
-- should be /ON THE LEFT/, that is, `M * x`
--
mds_matrix_M4_row_major :: [[Integer]]
mds_matrix_M4_row_major =
  [ [ 5 , 7 , 1 , 3 ]
  , [ 4 , 6 , 1 , 1 ]
  , [ 1 , 3 , 5 , 7 ]
  , [ 1 , 1 , 4 , 6 ]
  ]

-- from the paper
fast_mul_M4 :: Num a => [a] -> [a]
fast_mul_M4 [x0,x1,x2,x3] = [y0,y1,y2,y3] where
  t0 =   x0 + x1
  t1 =   x2 + x3
  t2 = 2*x1 + t1
  t3 = 2*x3 + t0
  t4 = 4*t1 + t3
  t5 = 4*t0 + t2
  t6 =   t3 + t5
  t7 =   t2 + t4
  ---
  y0 = t6
  y1 = t5
  y2 = t7
  y3 = t4

sanitCheck_mulM4 :: IO Bool
sanitCheck_mulM4 = do
  xs <- replicateM 4 $ randomRIO (-1000,1000) :: IO [Integer]
  let u = matrixMulTest mds_matrix_M4_row_major xs
  let v = fast_mul_M4 xs
  -- print xs
  -- print u
  -- print v
  return $ (u == v)

--------------------------------------------------------------------------------


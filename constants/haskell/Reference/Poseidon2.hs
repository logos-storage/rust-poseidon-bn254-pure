
module Reference.Poseidon2 where

--------------------------------------------------------------------------------

import Data.List
import Control.Monad

import Field

import HorizenLabs
import qualified HorizenLabs.Old as Old
import qualified HorizenLabs.New as New

--------------------------------------------------------------------------------

data ParamSet
  = OldParams
  | NewParams
  deriving (Eq,Show)

data Width 
  = T2 | T3 | T4 
  deriving (Eq,Show)

fromWidth :: Width -> Integer
fromWidth T2 = 2
fromWidth T3 = 3
fromWidth T4 = 4

data Instance = MkInstance
  { width    :: !Width
  , paramSet :: !ParamSet
  }
  deriving (Eq,Show)

allParamSets = [ OldParams, NewParams ]
allWidths    = [ T2, T3, T4 ]
allInstances = [ MkInstance w s | w<-allWidths, s<-allParamSets ]

checkPoseidon2 :: IO ()
checkPoseidon2 = do
  forM_ allInstances $ \which@(MkInstance width paramSet) -> do
    let t = fromWidth width
    let input  = map toF [0..t-1]
    let output = permute which input
    putStrLn "---------------------------------------"
    print which
    mapM_ print output 
    putStrLn $ "matches the KAT = " ++ show (output == kat which)

--------------------------------------------------------------------------------

externalMDS :: Instance -> [[F]]
externalMDS (MkInstance width _) = (map . map) toF $ case width of
  T2 -> mds_matrix_M2_row_major
  T3 -> mds_matrix_M3_row_major
  T4 -> mds_matrix_M4_row_major

internalDiag :: Instance -> [F]
internalDiag (MkInstance width paramSet) = map toF $ case paramSet of
  OldParams -> case width of
    T2 -> Old.diag_T2
    T3 -> Old.diag_T3
    T4 -> Old.diag_T4
  NewParams -> case width of
    T2 -> New.diag_T2
    T3 -> New.diag_T3
    T4 -> New.diag_T4

kat :: Instance -> [F]
kat (MkInstance width paramSet) = map toF $ case paramSet of
  OldParams -> case width of
    T2 -> Old.kat_T2
    T3 -> Old.kat_T3
    T4 -> Old.kat_T4
  NewParams -> case width of
    T2 -> New.kat_T2
    T3 -> New.kat_T3
    T4 -> New.kat_T4

roundConsts :: Instance -> [[F]]
roundConsts (MkInstance width paramSet) = (map . map) toF $ case paramSet of
  OldParams -> case width of
    T2 -> Old.roundConst_T2
    T3 -> Old.roundConst_T3
    T4 -> Old.roundConst_T4
  NewParams -> case width of
    T2 -> New.roundConst_T2
    T3 -> New.roundConst_T3
    T4 -> New.roundConst_T4

--------------------------------------------------------------------------------

type State = [F]
type Mtx   = [[F]]
type Diag  = [F]
type RCs   = [F]
type RC    = F

sbox :: F -> F
sbox x = x * x4 where
  x2 = x  * x
  x4 = x2 * x2

mdsMul :: Mtx -> State -> State
mdsMul matrix vector = [ sum (zipWith (*) column vector) | column <- matrix ]

diagMul :: Diag -> State -> State
diagMul diag vector = [ s + d * x | (d,x) <- zip diag vector ] where
  s = sum vector

splitRoundConsts :: [RCs] -> ( [RCs] , [RC] , [RCs] )
splitRoundConsts rows = ( initial , internal , final ) where
  n = length rows
  initial  = take    4  rows
  final    = drop (n-4) rows
  internal = map head $ take (n-8) $ drop 4 rows

internalRound :: Diag -> State -> RC -> State
internalRound diag old rc = new where
  (x:xs) = old
  x'     = sbox (x + rc)
  new    = diagMul diag (x':xs)

externalRound :: Mtx -> State -> RCs -> State
externalRound mds old rcs = new where
  xs' = zipWith (\x rc -> sbox (x + rc)) old rcs 
  new = mdsMul mds xs'

flipFoldl :: (b -> a -> b) -> [a] -> b -> b
flipFoldl f ys x = foldl' f x ys

permute :: Instance -> State -> State
permute which@(MkInstance width paramSet) input 
  | length input /= fromInteger (fromWidth width) = error "permute: invalid input dimensions"
  | otherwise = output
  where
    extMDS  = externalMDS  which
    intDiag = internalDiag which
    ( rcIni , rcMiddle , rcFinal ) = splitRoundConsts (roundConsts which)
    output = flipFoldl (externalRound extMDS ) rcFinal
           $ flipFoldl (internalRound intDiag) rcMiddle
           $ flipFoldl (externalRound extMDS ) rcIni
           $ mdsMul extMDS 
           $ input

compress :: Instance -> [F] -> F
compress which@(MkInstance width _paramset) input 
  | length input /= fromInteger (fromWidth width - 1)  = error "compress: invalid input dimensions"
  | otherwise = head (permute which $ input ++ [0])

--------------------------------------------------------------------------------

compressionTestCases :: IO ()
compressionTestCases = forM_ allInstances $ \which@(MkInstance width _paramset) -> do
  putStrLn "-----------------------"
  putStrLn $ "instance = " ++ show which
  let w = fromWidth width
  let input = map (*111) [1..w-1] :: [Integer]
  putStrLn $ "input = " ++ show input
  let hash = compress which (map toF input)
  putStrLn $ "hash  = " ++ show hash

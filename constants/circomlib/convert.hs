
-- script to convert the round constants to the desired format

module Main where

--------------------------------------------------------------------------------

import Data.List
import Control.Monad

import System.FilePath
import System.Directory

import Field
import Circomlib

--------------------------------------------------------------------------------

tgtDir :: FilePath
tgtDir = "out"

--------------------------------------------------------------------------------

integerToRust32 :: Integer -> String
integerToRust32 x = "Mont::unsafe_make( " ++ toMontString32 x ++ " )"

listToRust32 :: [Integer] -> [String]
listToRust32 xs = zipWith f prefixes xs ++ [close] where
  f p x = p ++ integerToRust32 x
  prefixes = "  [ " : repeat "  , "
  close    = "  ];"

--------------------------------------------------------------------------------

const_C :: Width -> [String]
const_C w = comment : def : ls where
  comment = "// round constants (t for external, 1 for internal rounds; flattened)"
  def     = "pub const CONST_C: [Mont; " ++ show len ++ "] = "
  xs      = poseidon_C w
  len     = length xs
  ls      = listToRust32 xs

const_M :: Width -> [String]
const_M w = comment : def : ls where
  comment = "// external mixing matrix (`t x t`, flattened)"
  def     = "pub const CONST_M: [Mont; " ++ show len ++ "] = "
  xs      = concat (poseidon_M w)
  len     = length xs
  ls      = listToRust32 xs

const_P :: Width -> [String]
const_P w = comment : def : ls where
  comment = "// internal mixing matrix (`t x t`s, flattened)"
  def     = "pub const CONST_P: [Mont; " ++ show len ++ "] = "
  xs      = concat (poseidon_P w)
  len     = length xs
  ls      = listToRust32 xs

const_S :: Width -> [String]
const_S w = comment : def : ls where
  comment = "// circomlib's optimization whatever"
  def     = "pub const CONST_S: [Mont; " ++ show len ++ "] = "
  xs      = poseidon_S w
  len     = length xs
  ls      = listToRust32 xs

--------------------------------------------------------------------------------

header :: Width -> [String]
header (MkW t) =
  [ ""
  , "// circomlib's Poseidon constants for t=" ++ show t 
  , ""
  , "use crate::bn254::montgomery::*;"
  , ""
  ]

sep :: [String] 
sep = 
  [ ""
  , "//------------------------------------------------------------------------------"
  , ""
  ]

rustSource :: Width -> String
rustSource t = unlines $ concat
  [ header  t , sep
  , const_C t , sep
  , const_M t , sep
  , const_P t , sep
  , const_S t , sep
  ]

writeRustConstants :: Width -> IO ()
writeRustConstants w@(MkW t) = do
  let fname = "t" ++ show t ++ ".rs"
  writeFile (tgtDir </> fname) (rustSource w)

main :: IO ()
main = do
  createDirectoryIfMissing False tgtDir
  forM_ [2..5] $ \t -> writeRustConstants (MkW t)

--------------------------------------------------------------------------------

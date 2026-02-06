
-- script to convert horizelabs' round constants to the desired format

module Main where

--------------------------------------------------------------------------------

import Data.List
import Control.Monad

import System.FilePath
import System.Directory

import Field
import Reference.Poseidon2

--------------------------------------------------------------------------------

tgtDirRoot :: FilePath
tgtDirRoot = "out"

tgtDirOld, tgtDirNew :: FilePath
tgtDirOld = tgtDirRoot </> "old"
tgtDirNew = tgtDirRoot </> "new"

--------------------------------------------------------------------------------

const_init :: Instance -> [String]
const_init which = comment : def : ls where
  comment = "// initial (external) round constants (flattened)"
  def     = "pub const INITIAL: [Mont; " ++ show len ++ "] = "
  xs      = map fromF $ concat ini
  len     = length xs
  ls      = listToRust32 xs
  (ini,_middle,_final) = splitRoundConsts (roundConsts which)

const_middle :: Instance -> [String]
const_middle which = comment : def : ls where
  comment = "// middle (internal) round constants"
  def     = "pub const INTERNAL: [Mont; " ++ show len ++ "] = "
  xs      = map fromF middle
  len     = length xs
  ls      = listToRust32 xs
  (_ini,middle,_final) = splitRoundConsts (roundConsts which)

const_final :: Instance -> [String]
const_final which = comment : def : ls where
  comment = "// final (external) round constants (flattened)"
  def     = "pub const FINAL: [Mont; " ++ show len ++ "] = "
  xs      = map fromF $ concat final
  len     = length xs
  ls      = listToRust32 xs
  (_ini,_middle,final) = splitRoundConsts (roundConsts which)

const_diag :: Instance -> [String]
const_diag which = comment : def : ls where
  comment = "// diagonal for the internal mixing matrix"
  def     = "pub const DIAGONAL: [Mont; " ++ show len ++ "] = "
  xs      = map fromF $ internalDiag which
  len     = length xs
  ls      = listToRust32 xs

const_kat :: Instance -> [String]
const_kat which = comment : def : ls where
  comment = "// known answer test"
  def     = "pub const KAT_MONT: [Mont; " ++ show len ++ "] = "
  xs      = map fromF $ kat which
  len     = length xs
  ls      = listToRust32 xs

--------------------------------------------------------------------------------

showWidth :: Instance -> String
showWidth (MkInstance width paramset) = "t = " ++ show (fromWidth width) 

showParamSet :: Instance -> String
showParamSet (MkInstance width paramset) = case paramset of
  OldParams -> "\"old\" set of constants"
  NewParams -> "\"new\" set of constants"

showInstance :: Instance -> String
showInstance which = "`" ++ showWidth which ++ "` (" ++ showParamSet which ++ ")"

header :: Instance -> [String]
header which =
  [ ""
  , "// HorizenLabs' Poseidon2 constants for " ++ showInstance which
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

rustSource :: Instance -> String
rustSource t = unlines $ concat
  [ header       t , sep
  , const_diag   t , sep
  , const_init   t , sep
  , const_middle t , sep
  , const_final  t , sep
  , const_kat    t , sep
  ]

writeRustConstants :: Instance -> IO ()
writeRustConstants which@(MkInstance width paramset) = do
  let fname = "t" ++ show (fromWidth width) ++ ".rs"
  let path = case paramset of
        OldParams -> tgtDirOld </> fname
        NewParams -> tgtDirNew </> fname
  print path
  writeFile path (rustSource which)

main :: IO ()
main = do
  createDirectoryIfMissing False tgtDirRoot
  createDirectoryIfMissing False tgtDirOld
  createDirectoryIfMissing False tgtDirNew
  forM_ allInstances $ \inst -> writeRustConstants inst

--------------------------------------------------------------------------------

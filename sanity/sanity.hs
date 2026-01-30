
-- some basic "sanity check"-type testing

{-# LANGUAGE PackageImports #-}

import Data.List
import Data.Word
import Data.Bits
import Text.Printf
import Control.Monad
import "random" System.Random

--------------------------------------------------------------------------------

u32_mask = 0xFFFF_FFFF

prime  = 21888242871839275222246405745257275088548364400416034343698204186575808495617
halfp1 = (div prime 2) + 1

r1     = 0x0e0a77c19a07df2f666ea36f7879462e36fc76959f60cd29ac96341c4ffffffb
r2     = 0x0216d0b17f4e44a58c49833d53bb808553fe3ab1e35c59e31bb8e645ae216da7
r3     = 0x0cf8594b7fcc657c893cc664a19fcfed2a489cbe1cfbb6b85e94d8e1b4bf0040

big1   = 0x3142edd042d60bd1c80322cb76b2f85cdc3f0f00b0151d71e4db8a57191eb5c7
big2   = 0x7d7cf5c0086f97e30ac1f5a6987ee9529c417d290b0c5fe203e4f3728cbc1b97
big3   = 0xbbfc0dc5195a120a2317351be185427ea931be03406c5062f9fff49bb1efa2d1

felt1  = 0x22ac1ee66024036b5ab6f194bf51bf7fd03cc3b9ca5c5b8a00d4796720dc4a9f
felt2  = 0x2063f06a7b59b4fa596d982aa362c094703bcf35eb5f4c171f9c48a6e34d39b8
felt3  = 0x07d89e99d3bbaebca574aefcf8f492c5188cc945728b9d4da1c33f8831bfd5d3

mont1  = 0x095fad8ecdabd8686a74e6c0c03384162216c484c62c6b29f7cb6585c7a4b4fc
mont2  = 0x01a6741e42e6d6bc5f830d505947fe1ef1226480a55c65600b62d2a19c974559
mont3  = 0x2e493a4b8bf5eb71facdfcdc73d11ffa9c15ed08671492cefbb9d8d024a2de63

hex :: Integer -> IO ()
hex = printf "0x%x\n"

--------------------------------------------------------------------------------

modp :: Integer -> Integer
modp x = mod x prime

neg x   = modp (negate x)
add x y = modp (x + y)
sub x y = modp (x - y)
mul x y = modp (x * y)

--------------------------------------------------------------------------------

twoTo32 :: Integer
twoTo32 = 2^32

-- compute Q such that `Q * P = -1 mod 2^32`
montQ :: Word32
montQ = base ^ expo where
  b    = 2^32 :: Integer
  base = (fromInteger $ mod (negate prime) b) :: Word32
  expo = (fromInteger $ div b 2 - 1         ) :: Word32

-- we need `Q * P = -1 mod 2^32`
checkQ :: Bool
checkQ = 0 == mod (1 + prime * fromIntegral montQ) twoTo32

--------------------------------------------------------------------------------

rndWord :: IO Word32
rndWord = randomIO

rndWords :: IO [Word32]
rndWords = replicateM 8 rndWord

showWord :: Word32 -> String
showWord = printf "0x%08x" 

showWords :: [Word32] -> String
showWords xs = "[ " ++ intercalate " , " (map showWord xs) ++ " ]"

printWords :: [Word32] -> IO ()
printWords = putStrLn . showWords

toWords :: Integer -> [Word32]
toWords = go 8 where
  go 0 0 = []
  go 0 _ = error "toWords: doesn't fit into 256 bits"
  go k x = fromInteger (x .&. u32_mask) : go (k-1) (shiftR x 32)

fromWords :: [Word32] -> Integer 
fromWords = go where
  go []     = 0
  go (x:xs) = fromIntegral x + shiftL (go xs) 32

printRndBig :: IO ()
printRndBig = do
  xs <- rndWords
  putStrLn (showWords xs)

rndMont :: IO [Word32]
rndMont = toWords <$> randomRIO (0,prime-1)

printRndMont :: IO ()
printRndMont = do
  xs <- rndMont
  putStrLn (showWords xs)

--------------------------------------------------------------------------------

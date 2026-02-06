
-- minimal code required to convert the Poseidon constants to the required format

module Field where

--------------------------------------------------------------------------------

import Data.Word
import Data.Bits
import Data.List
import Text.Printf

--------------------------------------------------------------------------------

u32_mask = 0x_FFFF_FFFF           :: Integer
u64_mask = 0x_FFFF_FFFF_FFFF_FFFF :: Integer

prime  = 21888242871839275222246405745257275088548364400416034343698204186575808495617

-- r1 = 0x0e0a77c19a07df2f666ea36f7879462e36fc76959f60cd29ac96341c4ffffffb
-- r2 = 0x0216d0b17f4e44a58c49833d53bb808553fe3ab1e35c59e31bb8e645ae216da7
-- r3 = 0x0cf8594b7fcc657c893cc664a19fcfed2a489cbe1cfbb6b85e94d8e1b4bf0040

--------------------------------------------------------------------------------

modp :: Integer -> Integer
modp x = mod x prime

neg x   = modp (negate x)
add x y = modp (x + y)
sub x y = modp (x - y)
mul x y = modp (x * y)

hex :: Integer -> IO ()
hex = printf "0x%x\n"

----------------------------------------

newtype F 
  = MkF Integer
  deriving (Eq)

toF :: Integer -> F
toF = MkF . modp

fromF :: F -> Integer
fromF (MkF x) = x

instance Show F where
  show (MkF x) = printf "0x%032x" x

instance Num F where
  fromInteger = toF
  negate x = MkF $ neg (fromF x)
  (+) x y  = MkF $ add (fromF x) (fromF y)
  (-) x y  = MkF $ sub (fromF x) (fromF y)
  (*) x y  = MkF $ mul (fromF x) (fromF y)
  abs      = error "abs"
  signum   = error "signum"

--------------------------------------------------------------------------------

showWord32 :: Word32 -> String
showWord32 = printf "0x%08x" 

showWords32 :: [Word32] -> String
showWords32 xs = "[ " ++ intercalate " , " (map showWord32 xs) ++ " ]"

printWords32 :: [Word32] -> IO ()
printWords32 = putStrLn . showWords32

toWords32 :: Integer -> [Word32]
toWords32 = go 8 where
  go 0 0 = []
  go 0 _ = error "toWords: doesn't fit into 256 bits"
  go k x = fromInteger (x .&. u32_mask) : go (k-1) (shiftR x 32)

fromWords32 :: [Word32] -> Integer 
fromWords32 = go where
  go []     = 0
  go (x:xs) = fromIntegral x + shiftL (go xs) 32

--------------------------------------------------------------------------------

toMontgomery :: Integer -> Integer
toMontgomery x = modp (2^256 * x)

toMontString32 :: Integer -> String
toMontString32 = showWords32 . toWords32 . toMontgomery 

--------------------------------------------------------------------------------

integerToRust32 :: Integer -> String
integerToRust32 x = "Mont::unsafe_make( " ++ toMontString32 x ++ " )"

listToRust32 :: [Integer] -> [String]
listToRust32 xs = zipWith f prefixes xs ++ [close] where
  f p x = p ++ integerToRust32 x
  prefixes = "  [ " : repeat "  , "
  close    = "  ];"

--------------------------------------------------------------------------------

A | 0 0 1 1
B | 0 1 0 1
------------
  | 0 0 0 0 | Const 0
  | 0 0 0 1 | And
  | 0 0 1 0 | A not-implies B
  | 0 0 1 1 | A
  | 0 1 0 0 | B not-implies A
  | 0 1 0 1 | B
  | 0 1 1 0 | XOR
  | 0 1 1 1 | Or
  | 1 0 0 0 | Nor
  | 1 0 0 1 | XNor (Eq)
  | 1 0 1 0 | ~B
  | 1 0 1 1 | A implies B
  | 1 1 0 0 | ~A
  | 1 1 0 1 | B implies A
  | 1 1 1 0 | Nand
  | 1 1 1 1 | Const 1

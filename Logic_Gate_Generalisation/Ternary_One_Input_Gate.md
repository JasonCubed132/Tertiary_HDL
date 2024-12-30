A | 0 1 2
----------
  | 0 0 0 | Const 0 or Min'ing Minus 2
  | 0 0 1 | Min'ing Minus 1 or 1 if A == 2
  | 0 0 2 | 2 if A == 2, 0 otherwise
  | 0 1 0 | 1 if A == 1, 0 otherwise
  | 0 1 1 | 0 if A == 0, 1 otherwise
  | 0 1 2 | Id
  | 0 2 0 | 2 if A == 1, 0 otherwise
  | 0 2 1 | 
  | 0 2 2 | 0 if A == 0, 2 otherwise
  | 1 0 0 | 1 if A == 0, 0 otherwise
  | 1 0 1 | 0 if A == 1, 1 otherwise
  | 1 0 2 | 
  | 1 1 0 | 0 if A == 2, 1 otherwise
  | 1 1 1 | Const 1
  | 1 1 2 | 2 if A == 2, 1 otherwise
  | 1 2 0 | Circular Minus 2 or Circular Add 1
  | 1 2 1 | 2 if A == 1, 1 otherwise
  | 1 2 2 | Add 1 or 1 if A == 0, 2 otherwise
  | 2 0 0 | 2 if A == 0, 0 otherwise
  | 2 0 1 | Circular Minus 1 or Circular Add 2
  | 2 0 2 | 0 if A == 1, 2 otherwise
  | 2 1 0 | Not
  | 2 1 1 | 2 if A == 0, 1 otherwise
  | 2 1 2 | 1 if A == 1, 2 otherwise
  | 2 2 0 | 0 if A == 2, 2 otherwise
  | 2 2 1 | 1 if A == 2, 2 otherwuse
  | 2 2 2 | Const 2 or Max'ing Add 2

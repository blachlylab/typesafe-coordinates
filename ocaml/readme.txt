Here is a basic proof-of-concept of Typesafe Coordinate Systems (Gregory and Blachly, 2021)
implemented in OCaml. Limitations in this implementation include:

1. Without the use of macros (PPX?) or other metaprogramming facilities,
    conversion functions must be each written out explicitly
    a. I have taken the approach of writing only 4 conversion functions (instead of 12?)
        to convert only basis OR openness
2. OCaml does not have function overloading or ad hoc polymorphism, so `overlaps` must
    be defined separately as `overlaps_open` and `overlaps_closed` according to type
    (while still remaining generic over basis)
3. The same would be true for e.g. `contains` or other functions needing to vary
    behavior with respect to one or both type parameters


```
let i = make_zbho 0 100
let j = make_zbho 50 150
let k = make_zbho 100 200

assert (overlaps_open i j)
assert (overlaps_open j k)
assert (not (overlaps_open i k))

(* compilation error: overlaps_closed i j *)

let l = make_obho 100 200
(* compilation error: overlaps_open k l *)

(* You could convert only basis (or openness) *)
let i_conv = ob_interval_of_zb i
val i_conv : (ob, ho) interval = {x = 1; y = 101}

(* Or chain function application to convert basis and openness *)
let j_conv = cl_interval_of_ho (ob_interval_of_zb j)
val j_conv : (ob, cl) interval = {x = 51; y = 150}
```

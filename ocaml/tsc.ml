(** Typesafe Coordinate Systems proof of concept for OCaml.

    Note that I learned OCaml for purposes of this excercise; constructive criticism welcomed.

    @author James S. Blachly, MD
*)

(* Originally, we defined types zbho, zbc, obho, obc and paramterized polymorphic interval
    type only once. However as some functions need to be specialized only on basis OR openness,
    and because there is no compile time type introspection/static if AFAICT, it's less duplication
    to define (and parameterize upon) the basis and openness types separately. *)

type zb (** zero-based *)
type ob (** one-based *)
type ho (** open *)
type cl (** closed *)

(* NB: For maximal compatibility and applicability,
    genome positions should be 64 bit, which unfortunately
    in OCaml requires a library solution *)

type 'a coordinate = {x: int}
type ('a, 'b) interval = {x: int; y: int}
type ('a, 'b) interval_composed = {start_pos: 'a coordinate; end_pos: 'a coordinate}

(** Construct a coordinate position of specific type *)
let make_zb x : zb coordinate = {x}
let make_ob x : ob coordinate = {x}

(** Construct a coordinate interval of specific type *)
let make_zbho x y : (zb, ho) interval = {x; y}
let make_zbc  x y : (zb, cl) interval = {x; y}
let make_obho x y : (ob, ho) interval = {x; y}
let make_obc  x y : (ob, cl) interval = {x; y}

let ident i: ('a,'b) interval = i

(** define i < j for intervals i,j iff i.start < j.start *)
let lt (i: ('a, 'b) interval) (j: ('a, 'b) interval) : bool =
    if i.x < j.x then true else false

(** define i <= j for intervals i,j iff i>start <= j.start *)
let lte (i: ('a, 'b) interval) (j: ('a, 'b) interval) : bool =
    if i.x <= j.x then true else false

(** i > j and i >= j should be logical inverse of `lte` and `lt` respectively *)
let gt (i: ('a, 'b) interval) (j: ('a, 'b) interval) : bool = not (lte i j)
let gte (i: ('a, 'b) interval) (j: ('a, 'b) interval) : bool = not (lt i j)

(** Infix operator equivalents of lt, lte, gt, gte *)
let (<*) i j = lt i j
let (<=*) i j = lte i j
let (>*) i j = gt i j
let (>=*) i j = gte i j

(* Now, interval arithmetic does not differ whether you count
    from zero (zb) or from one (ob). However, the openness
    or closed-ness of the interval end does make a difference
    in the inequalities used, so we will provide two polymorphic
    versions specialized on openness.

    Note also that we must explicitly give type 'a for both i and j
    to ensure we are not comparing across different basis

    Finally, sadly OCaml does not support function overloading
    (alternatively, ad hoc polymorphism) thus two distinct function names.
    One potential solution is the object system and module system.
    We leave this exercise to the reader.
*)

let overlaps_open (i: ('a, ho) interval) (j: ('a, ho) interval) : bool =
    let i,j =
        if i >* j then j,i else i,j in
    if i.y <= j.x then false else true  (* <= in half open range *)

let overlaps_closed (i: ('a, cl) interval) (j: ('a, cl) interval) : bool =
    let i,j =
        if i >* j then j,i else i,j in
    if i.y < j.x then false else true (* strictly < in closed range *)


type 'a t

val create : int -> 'a t
val insert : 'a t -> 'a -> unit
val mem : 'a t -> 'a -> bool
val remove : 'a t -> 'a -> unit

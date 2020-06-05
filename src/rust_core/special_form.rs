/*

Clojure Special Forms

(pprint (keys (. clojure.lang.Compiler specials)))

TODO: &
TODO: monitor-exit
TODO: case*
TODO: try
TODO: reify*
TODO: finally
TODO: loop*
TODO: do
TODO: letfn*
TODO: if
TODO: clojure.core/import*
TODO: new
TODO: deftype*
TODO: let*
- fn*
TODO: recur
TODO: set!
TODO: .
TODO: var
TODO: quote
TODO: catch
TODO: throw
TODO: monitor-enter
TODO: def

*/

pub(crate) mod fn_star;
pub use self::fn_star::*;

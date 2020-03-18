;Given a valid (IPv4) IP address, return a defanged version of that IP address.

;A defanged IP address replaces every period "." with "[.]".

;Example 1:

;Input: address = "1.1.1.1"
;Output: "1[.]1[.]1[.]1"

;Example 2:

;Input: address = "255.100.50.0"
;Output: "255[.]100[.]50[.]0"

(require '[clojure.string :as str])

; time O(n)
; space O(2n) -> O(n)
(defn defang [ip]
  (str/join "[.]" (str/split ip #"\.")))

(println (defang "1.1.1.1"))
(println (defang "255.100.50.0"))
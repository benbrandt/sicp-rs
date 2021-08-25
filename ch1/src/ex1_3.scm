
(define (square x) (* x x))

(define (sum-of-squares x y)
    (+ (square x) (square y)))

(define (sum-of-largest-squares x y z)
    (cond
        ((and (>= x z) (>= y z))
            (sum-of-squares x y))
        ((and (>= y x) (>= z x))
            (sum-of-squares y z))
        ((and (>= x y) (>= z y))
            (sum-of-squares x z))))
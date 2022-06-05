(define (cbrt-iter guess x)
    (if (good-enough? guess x)
        guess
        (cbrt-iter (improve guess x) x)))

(define (improve guess x)
    (/ (+ (/ x (* guess guess))
          (* 2 guess))
        3))

(define (good-enough? guess x)
    (= (improve guess x) guess))

(define (cbrt x)
    (cbrt-iter 1.1 x))
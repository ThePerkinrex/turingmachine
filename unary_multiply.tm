EMPTY: #
INITIAL_STATE: q0

(q0, 1): (q1, 0, R)

(q1, 1): (q1, 1, R)
(q1, x): (q2, x, R)

(q2, 1): (q3, 0, R)
(q2, =): (q5, =, L)

(q3, 1): (q3, 1, R)
(q3, =): (q3, =, R)
(q3, #): (q4, 1, L)

(q4, 1): (q4, 1, L)
(q4, =): (q4, =, L)
(q4, 0): (q2, 1, R)

(q5, 1): (q5, 1, L)
(q5, x): (q5, x, L)
(q5, 0): (q0, 1, R)
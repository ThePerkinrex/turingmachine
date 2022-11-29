EMPTY: #
INITIAL_STATE: q0

(q0, e): (q1, E, L)

(q1, e): (q1, e, L)
(q1, #): (q2, #, L)

(q2, 1): (q2, 0, L)
(q2, 0): (q3, 1, R)
(q2, #): (q3, 1, R)

(q3, 0): (q3, 0, R)
(q3, #): (q4, #, R)

(q4, e): (q4, e, R)
(q4, E): (q0, e, R)

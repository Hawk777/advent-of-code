#!/usr/bin/env python3


def ext_euclid(a, b):
    """
    Perform the extended Euclidean algorithm.

    Return the GCD and the two Bézout coefficients.
    """
    s = 0
    old_s = 1
    r = b
    old_r = a

    while r != 0:
        quotient = old_r // r
        (old_r, r) = (r, old_r - quotient * r)
        (old_s, s) = (s, old_s - quotient * s)
    
    if b != 0:
        bezout_t = (old_r - old_s * a) // b
    else:
        bezout_t = 0

    return old_r, old_s, bezout_t


def crt(eqns):
    """
    Perform the Chinese remainder theorem.

    :params eqn: a list of equations, each of which is a tuple of (ai, ni),
    representing the equation x = ai (mod ni)

    Return the solution to the system of equations.
    """
    assert len(eqns) >= 2
    a1, n1 = eqns[0]
    a2, n2 = eqns[1]
    gcd, m1, m2 = ext_euclid(n1, n2)
    assert gcd == 1
    x = a1 * m2 * n2 + a2 * m1 * n1
    if len(eqns) == 2:
        return x
    else:
        return crt(eqns[2:] + [(x, n1 * n2)])

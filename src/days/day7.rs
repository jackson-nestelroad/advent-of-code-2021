use crate::common::{iAoc, AocResult, IntoAocResult};
use num::Integer;

fn parse_input(input: &str) -> AocResult<Vec<i32>> {
    input
        .split(',')
        .map(|num| num.parse::<i32>())
        .collect::<Result<_, _>>()
        .into_aoc_result()
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let mut positions = parse_input(input.trim())?;

    positions.sort();
    let mid = positions.len() / 2;
    let median = positions[mid];

    let result: i32 = positions.into_iter().map(|pos| (pos - median).abs()).sum();

    Ok(result as iAoc)
}

fn calculate_fuel_cost(steps: i32) -> i32 {
    (steps * (steps + 1)) / 2
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let positions = parse_input(input.trim())?;

    /*

        Let C be the set of crab positions, where c is a single position.
        Let f(t) be the fuel cost function with respect to the target value t.

            f(t) = sum_c ((|c-t|) (|c-t|+1) / 2) = 1/2 * sum_c ((|c-t|) (|c-t|+1))

        Make f(t) a piecewise function, to remove the absolute value.

            f(t) = 1/2 * sum_c {
                ((t-c) (t-c+1))     ; t >= c
                ((c-t) (c-t+1))     ; t < c
            }

            f(t) = 1/2 * sum_c {
                (c^2 - 2ct + c + t^2 - t)   ; t >= c
                (t^2 - 2ct + t + c^2 - c)   ; t < c>
            }

        To minimize t, we use the first derivate of f(t) with respect to t.

            d/dt(f(t)) = 1/2 * sum_c {
                2t - 2c - 1         ; t >= c
                2t - 2c + 1         ; t < c
            }

            d/dt(f(t)) = 1/2 * sum_c ((2t - 2c) + {
                -1      ; t >= c
                1       ; t < c
            })

            d/dt(f(t)) = 1/2 (sum_c (2t) - sum_c (2c) + sum_c {
                -1      ; t >= c
                1       ; t < c
            })

            d/dt(f(t)) = 1/2 (|C|*2t - 2 * sum_c (c) + sum_c {
                -1      ; t >= c
                1       ; t < c
            })

        We can remove the piecewise section by introducing two new values:
            m(t) => number of values in C less than t
            n(t) => number of values in C greater than or equal to t

            d/dt(f(t)) = 1/2 (|C|*2t - 2 * sum_c (c) + m(t) - n(t))

        We can minimize t by setting the derivative equal to 0 and solve for t.

            0 = 1/2 (|C|*2t - 2 * sum_c (c) + m(t) - n(t))

            0 = |C|*2t - 2 * sum_c (c) + m(t) - n(t)

            |C|*2t = 2 * sum_c (c) + n(t) - m(t)

            t = (sum_c (c))/|C| + (n(t) - m(t))/2|C|

        Note that (sum_c (c))/|C| is the average of values in C.

            t = avg + (n(t) - m(t))/2|C|

        We unfortunately still have values that are dependent on t when solving for t.
        However, we can restrict this value based on their definition.

            0 <= m(t) <= |C|
            0 <= n(t) <= |C|
            n(t) + m(t) = |C|

            -|C| <= n(t) - m(t) <= |C|
            -1/2 <= (n(t) - m(t))/2|C| <= 1/2

        Thus,
            t = avg + k, where -1/2 <= k <= 1/2

        So, t is close to the average.

        Because we are working with integers, we only need to check two integer values, which is
        the floor and ceiling of the average. This can be discovered intuitively:
            subtracting 1/2 = floor(integer avg)
            adding 1/2 = ceil(integer avg)


        The below solution uses this result attempts two integer values based on this range.

    */

    let min = positions
        .iter()
        .sum::<i32>()
        .div_floor(&(positions.len() as i32));
    let max = min + 1;

    let result1: i32 = positions
        .iter()
        .map(|pos| calculate_fuel_cost((pos - min).abs()))
        .sum();
    let result2: i32 = positions
        .into_iter()
        .map(|pos| calculate_fuel_cost((pos - max).abs()))
        .sum();
    let result = result1.min(result2);

    Ok(result as iAoc)
}

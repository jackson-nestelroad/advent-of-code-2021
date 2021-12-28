use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use std::str::FromStr;

/// The variables used by the MONAD.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Variable {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl FromStr for Variable {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(AocError::new("invalid variable")),
        }
    }
}

/// A parameter to an instruction.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Parameter {
    Variable(Variable),
    Literal(i64),
}

impl FromStr for Parameter {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Variable::from_str(input) {
            Ok(var) => Ok(Self::Variable(var)),
            Err(_) => Ok(Self::Literal(
                input
                    .parse::<i64>()
                    .into_aoc_result_msg("invalid integer literal")?,
            )),
        }
    }
}

/// A single instruction.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Parameter),
    Mul(Variable, Parameter),
    Div(Variable, Parameter),
    Mod(Variable, Parameter),
    Eql(Variable, Parameter),
}

/// Parse all instructions from the input string.
fn parse_instructions(input: &str) -> AocResult<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            match split.next().into_aoc_result()? {
                "inp" => Ok(Instruction::Inp(Variable::from_str(
                    split.next().into_aoc_result()?,
                )?)),
                "add" => Ok(Instruction::Add(
                    Variable::from_str(split.next().into_aoc_result()?)?,
                    Parameter::from_str(split.next().into_aoc_result()?)?,
                )),
                "mul" => Ok(Instruction::Mul(
                    Variable::from_str(split.next().into_aoc_result()?)?,
                    Parameter::from_str(split.next().into_aoc_result()?)?,
                )),
                "div" => Ok(Instruction::Div(
                    Variable::from_str(split.next().into_aoc_result()?)?,
                    Parameter::from_str(split.next().into_aoc_result()?)?,
                )),
                "mod" => Ok(Instruction::Mod(
                    Variable::from_str(split.next().into_aoc_result()?)?,
                    Parameter::from_str(split.next().into_aoc_result()?)?,
                )),
                "eql" => Ok(Instruction::Eql(
                    Variable::from_str(split.next().into_aoc_result()?)?,
                    Parameter::from_str(split.next().into_aoc_result()?)?,
                )),
                _ => Err(AocError::new("invalid instruction")),
            }
        })
        .collect::<Result<_, _>>()
}

/// Runs the MONAD program with the given digits as input.
///
/// My first solution was to attempt to binary search for the correct input
/// between the numbers 11111111111111 and 99999999999999. Unfortunately,
/// the MONAD is not monotonic, so this solution is incorrect.
///
/// My second solution was to just try every possible input starting from
/// the maximum. Of course, this solution is much too slow due to the
/// potential number of inputs (9^14).
///
/// Now, running the MONAD is only used to verify the problem solution.
fn run_monad(instructions: &Vec<Instruction>, input: &[u8; 14]) -> bool {
    fn param_value(param: &Parameter, vars: &[i64; 4]) -> i64 {
        match param {
            Parameter::Variable(var) => vars[*var as usize],
            Parameter::Literal(literal) => *literal,
        }
    }
    let mut i = 0;
    let mut vars = [0i64; 4];
    for instruction in instructions {
        match instruction {
            Instruction::Inp(var) => {
                vars[*var as usize] = input[i] as i64;
                i += 1;
            }
            Instruction::Add(var, param) => {
                vars[*var as usize] = vars[*var as usize] + param_value(param, &vars);
            }
            Instruction::Mul(var, param) => {
                vars[*var as usize] = vars[*var as usize] * param_value(param, &vars);
            }
            Instruction::Div(var, param) => {
                vars[*var as usize] = vars[*var as usize] / param_value(param, &vars);
            }
            Instruction::Mod(var, param) => {
                vars[*var as usize] = vars[*var as usize] % param_value(param, &vars);
            }
            Instruction::Eql(var, param) => {
                vars[*var as usize] = if vars[*var as usize] == param_value(param, &vars) {
                    1
                } else {
                    0
                };
            }
        }
    }

    vars[Variable::Z as usize] == 0
}

/*

    Solving this problem requires an analysis of the MONAD, which is the input to
    the problem.

    The MONAD has a single subroutine that is called 14 times, or once for each
    digit. This pattern can easily be identified by looking at the 14 occurrences
    of the `inp w` instruction.

    The subroutine has 18 instructions. For instance:

        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 12
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 6
        mul y x
        add z y

    Each subroutine call has the same instructions, but a few of the parameters
    are adjusted depending on the call. Here is the parameterized and commented
    version:

        # monad_subroutine(pop_stack, stack_pop_add, stack_push_add)
        # BEGIN
        # Read the next digit into w
        inp w

        # Read the value at the top of the stack
        mul x 0
        add x z
        mod x 26

        # Optionally pop the value off of the stack
        div z %if pop_stack { 26 } else { 1 }%

        # x = popped_digit + stack_pop_add
        add x %stack_pop_add%

        # x = popped_digit + stack_pop_add == current_digit
        eql x w

        # x = popped_digit + stack_pop_add != current_digit
        eql x 0

        # At this point, x is 0 or 1 depending on the above condition.
        # We'll let this condition be named `should_push`.

        # y = 25
        mul y 0
        add y 25

        # y = if should_push { 25 } else { 0 }
        mul y x

        # y = if should_push { 26 } else { 1 }
        add y 1

        # z = if should_push { 26 * z } else { z }
        mul z y

        # y = current_digit + stack_push_add
        mul y 0
        add y w
        add y %stack_push_add%

        # y = if should_push { current_digit + stack_push_add } else { 0 }
        mul y x

        # z =
            # if should_push { 26 * z + (curent_digit + stack_push_add) }
            # else { z }
        add z y

        # END


    We can now see more clearly how this program works. Variable w always holds
    the current digit. Variable x holds the `should_push` condition, which
    represents if the current subroutine call should push a new value to the
    stack. Variable y is a temporary variable that is repeatedly reset and simply
    holds intermediate values. Finally, variable z is the aforementioned stack
    of digits and offsets.

    The stack in variable z works like a number in base-26:

        z % 26 => the last base-26 digit of z, which is between 0 and 25
        26 * z + d => base-26 left shift z and add a new digit to the right

    This means that d, or the next value on the stack, must be less than 26.
    Since d = current_digit + stack_push_add, and current_digit is in the range
    of 0 to 9, stack_push_add <= 16.

    The top of the stack is always examined at the beginning of the program, but
    this value is not always popped. This is the first parameter: `pop_stack`.
    The other two variables, `stack_pop_add` and `stack_push_add`, are two offset
    integers for comparing the current digit to the value on the top of the stack
    and pushing hte current_digit to the top of the stack.

    Thus, the MONAD program can be summarized as follows:
        1. Get the next digit as input.
        2. Get the value at the top of the stack, optionally popping it out.
        3. Push (current_digit + stack_push_add) to the top of the stack if
            popped_digit + stack_pop_add != current_digit.


    Another abstraction can be identified that reduces the number of parameters
    down to 2 (but it is not necessarily required). The subroutine call pops from
    the stack iff stack_pop_add is negative, and it does not pop from the stack
    iff stack_pop_add is positive. Thus, the pop_stack parameter can be removed,
    and the sign of the stack_pop_add parameter can be checked instead.

    Furthermore, based on the actual input, when stack_pop_add is positive, it is
    always greater than or equal to 10. Reviewing the condition for pushing from
    above:

        should_push = popped_digit + stack_pop_add != current_digit

    The current digit, which is between 0 and 9, can never be equal to the left
    side of the equation, because it is at least 10! Thus, when stack_pop_add is
    positive, should_push will always be true, and the subroutine call will always
    push the next value.

    Things obviously get interesting when stack_pop_add is negative, since a
    value is popped from the stack and the should_push condition can actually be
    false.


    The MONAD accepts a model number if it finishes execution with z == 0, or
    when the stack is empty. Thus, there must be an even number of pushes and
    pops. Based on our previous observations, this is easy to verify. Subroutine
    calls with stack_pop_add > 0 will always push a new value to the stack, and
    calls with stack_pop_add < 0 will only push a new value to the stack if the
    condition is met.

    Analyzing the input once again, there are conveniently 7 always-pushing calls
    and 7 popping-and-maybe-pushing calls. Thus, to make things even, we must
    assure that the 7 popping-and-maybe-pushing calls never actually push their
    value, so:

        popped_digit + stack_pop_add == current_digit

    We can use this information, alongside the order of subroutine calls, to
    create a series of relationships between two digits of a model number, and we
    can find exactly which numbers will be accepted by the program.


    A digit relationship will be represented in the form digit[a] + C = digit[b],
    where a, b are digit indicies between 0 and 13, and C is a constant integer.
    All digit relationships come from the above condition that makes a subroutine
    not push a new value to the stack:

        popped_digit + stack_pop_add == current_digit
        digit[a]     + C             == digit[b]

    Due to the stack_push_add parameter that is used to when pushing digits onto
    the stack, C = b.stack_pop_add - a.stack_push_add.


    The following code shows how subroutine calls and digit relationships are
    represented and parsed.

*/

/// A call to the 18-instruction subroutine in the MONAD, which takes two parameters.
#[derive(Debug)]
struct MonadSubroutineCall {
    stack_pop_add: i32,
    stack_push_add: i32,
}

impl MonadSubroutineCall {
    pub fn new(stack_pop_add: i32, stack_push_add: i32) -> Self {
        Self {
            stack_pop_add,
            stack_push_add,
        }
    }
}

/// Parses the MONAD into even groups of subroutine calls, which make up the entire
/// program.
fn parse_monad_subroutines(monad: &Vec<Instruction>) -> AocResult<Vec<MonadSubroutineCall>> {
    monad
        .iter()
        .chunks(18)
        .into_iter()
        .map(|subroutine| {
            let mut subroutine = subroutine.skip(5);
            let stack_pop_add = match subroutine.next() {
                Some(Instruction::Add(_, Parameter::Literal(num))) => num,
                _ => return Err(AocError::new("invalid stack peek addition instruction")),
            };
            let mut subroutine = subroutine.skip(9);
            let stack_push_add = match subroutine.next() {
                Some(Instruction::Add(_, Parameter::Literal(num))) => num,
                _ => return Err(AocError::new("invalid stack push addition instruction")),
            };
            Ok(MonadSubroutineCall::new(
                *stack_pop_add as i32,
                *stack_push_add as i32,
            ))
        })
        .collect::<Result<_, _>>()
}

/// Represents a relationship between two digits of the model number.
#[derive(Debug)]
struct DigitRelationship {
    a: usize,
    b: usize,
    c: i8,
}

impl DigitRelationship {
    pub fn new(a: usize, b: usize, c: i8) -> Self {
        Self { a, b, c }
    }
}

/// Parses MONAD subroutine calls into the corresponding digit relationships.
fn parse_digit_relationships(
    subroutine_calls: Vec<MonadSubroutineCall>,
) -> AocResult<Vec<DigitRelationship>> {
    // Emulate the stack of digits. Instead of storing an actual digit, we store
    // the digit index, which represents any digit that may be passed in at this
    // position.
    let mut stack = Vec::new();
    let mut relationships = Vec::new();
    for (digit_index, subroutine_call) in subroutine_calls.into_iter().enumerate() {
        if subroutine_call.stack_pop_add >= 0 || stack.is_empty() {
            // Always-pushing call.
            stack.push((digit_index, subroutine_call.stack_push_add));
        } else {
            // Popping call, make sure it doesn't push by adding a digit relationship.
            let (popped_digit_index, stack_push_add) = stack.pop().unwrap();
            relationships.push(DigitRelationship::new(
                popped_digit_index,
                digit_index,
                (stack_push_add + subroutine_call.stack_pop_add) as i8,
            ))
        }
    }

    if !stack.is_empty() {
        // If the stack is not empty at the end of this simulation, there is no hope.
        // There are too many always-pushing calls and not enough pops.
        Err(AocError::new("stack is not empty at end of execution"))
    } else {
        Ok(relationships)
    }
}

/*
    At this point, a series of digit relationships (specifically 7) are known.

    Maximization:

        digit[a] + C = digit[b]
            digit[b] = 9
            digit[a] = digit[b] - C = 9 - C


        digit[a] - C = digit[b]
            digit[a] = 9
            digit[b] = digit[a] - C = 9 - C

    Minimization:

        digit[a] + C = digit[b]
            digit[a] = 1
            digit[b] = digit[a] + C = 1 + C

        digit[a] - C = digit[b]
            digit[b] = 1
            digit[a] = digit[b] + C = 1 + C
*/

fn maximize_digits(digit_relationships: Vec<DigitRelationship>) -> [u8; 14] {
    let mut digits = [9u8; 14];
    for DigitRelationship { a, b, c } in digit_relationships {
        if c > 0 {
            digits[a] -= c as u8;
        } else {
            digits[b] -= (-c) as u8;
        }
    }
    digits
}

fn minimize_digits(digit_relationships: Vec<DigitRelationship>) -> [u8; 14] {
    let mut digits = [1u8; 14];
    for DigitRelationship { a, b, c } in digit_relationships {
        if c > 0 {
            digits[b] += c as u8;
        } else {
            digits[a] += (-c) as u8;
        }
    }
    digits
}

/// Joins an array of digits back into the number it represents.
fn join_digits(digits: &[u8; 14]) -> u64 {
    digits
        .iter()
        .fold(0u64, |acc, digit| 10 * acc + *digit as u64)
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let monad = parse_instructions(input)?;
    let subroutine_calls = parse_monad_subroutines(&monad)?;
    let digit_relationships = parse_digit_relationships(subroutine_calls)?;
    let digits = maximize_digits(digit_relationships);

    if !run_monad(&monad, &digits) {
        Err(AocError::new("maximized digits do not pass the program"))
    } else {
        let result = join_digits(&digits);
        Ok(result as iAoc)
    }
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let monad = parse_instructions(input)?;
    let subroutine_calls = parse_monad_subroutines(&monad)?;
    let digit_relationships = parse_digit_relationships(subroutine_calls)?;
    let digits = minimize_digits(digit_relationships);

    if !run_monad(&monad, &digits) {
        Err(AocError::new("minimized digits do not pass the program"))
    } else {
        let result = join_digits(&digits);
        Ok(result as iAoc)
    }
}

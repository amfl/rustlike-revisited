use std::num::Wrapping;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// A pseudo-random number generator based on the technique of
/// M.E. O'Neill. See pcg-random.org for a lengthy description of the benefits
/// of this generator over other possible techniques.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PCGen {
    /// Our "position" within the number sequence.
    state: Wrapping<u64>,

    /// The number sequence that we're within. This value must be odd.
    inc: Wrapping<u64>,
}

impl PCGen {
    /// Given a state and incriment value, creates for you a new PCGen
    /// value. Most state and inc values that humans pick by hand end up
    /// having 0 as the first result of `next()` because humans don't tend to
    /// hand pick values that are big enough to actually use most of the bits
    /// of a `u64`. Because of this, we automatically call `next()` once for
    /// you and that's the generator that you get.
    ///
    /// The basic idea is that the `state` determines *where* in a number
    /// stream that you are, and then the `inc` value determines *which*
    /// number stream you're actually within. Two generators with the same
    /// `inc` value will produce the same number stream, offset by some (hard
    /// to predict) amount based on their initial states. However, two
    /// generators with different `inc` values will produce their values in a
    /// totally different order even given the same initial `state`.
    ///
    /// For the generator to work properly, the `inc` value must always be an
    /// odd value. As a result, any even input given for `inc` will be bumped
    /// up to the next odd value. This still allows you up to 2**63 possible
    /// number streams that are each 2**64 results long before they begin to
    /// loop, which is really quite a bit.
    pub fn new(new_state: u64, new_inc: u64) -> PCGen {
        let mut result = PCGen {
            state: Wrapping(new_state),
            inc: Wrapping(new_inc) | Wrapping(1),
        };
        result.next();
        result
    }

    // TODO: A way to save/restore a PCGen value.

    /// Produces the next 32 bits of output from this generator. Of course,
    /// for some data types, you may need to call this more than once (such as
    /// two calls to form a u64), or you could even break up this 32 bit
    /// sequence into more than one value (such as making a 32 element array
    /// of bools).
    pub fn next(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate * Wrapping(6364136223846793005) + self.inc;
        let Wrapping(xorshifted_sixfour) = ((oldstate >> 18) ^ oldstate) >> 27;
        let Wrapping(rot_sixfour) = oldstate >> 59;
        let xorshifted = xorshifted_sixfour as u32;
        let rot = rot_sixfour as u32;
        let Wrapping(negated_rot) = -Wrapping(rot);
        xorshifted >> rot | (xorshifted << (negated_rot & 31))
    }

    /// A helper method that produces a value in the range
    ///
    /// `low <= result < high`
    ///
    /// If the `low` value is equal to or greater than the `high` value this
    /// method will panic.
    ///
    /// The output uses `%`, so the distribution is only even when the range
    /// (high-low) is some power of 2. Otherwise some part of the output range
    /// will be slightly more likely than the rest.
    pub fn in_range(&mut self, low: i32, high: i32) -> i32 {
        if low < high {
            let out = self.next();
            let range = (high - low) as u32;
            (out % range) as i32 + low
        } else {
            panic!("bad inputs to in_range: low={}, high={}", low, high)
        }
    }

    /// Constructs a new PCGen value by using the current system time to seed
    /// the generator. For cryptographic work this isn't very good, because
    /// the system time can be guessed of course. However, for most other
    /// purposes this is probably sufficient.
    pub fn from_time() -> PCGen {
        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(elapsed) => {
                let seconds = elapsed.as_secs();
                PCGen::new(seconds, seconds)
            }
            Err(elapsed_error) => {
                let seconds = elapsed_error.duration().as_secs();
                PCGen::new(seconds, seconds)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PCGen;
    use std::cmp::Ordering;
    use std::num::Wrapping;

    /// The inc value of a PCGen built with `new` must always be odd.
    quickcheck!{
        fn new_inc_is_always_odd(s: u64, i: u64) -> bool {
            let gen = PCGen::new(s,i);
            let Wrapping(actual_inc) = gen.inc;
            actual_inc % 2 == 1
        }
    }

    /// The inc value of a generator must not be changed by the `next` method.
    quickcheck!{
        fn inc_never_changes(s: u64, i: u64) -> bool {
            let gen = PCGen::new(s,i);
            let mut gen2 = gen.clone();
            gen2.next();
            gen.inc == gen2.inc
        }
    }

    /// The `in_range` method must never generate a value that's outside the
    /// range specified.
    quickcheck!{
        fn in_range_always_gives_good_output(s: u64, i: u64, a: i32, b: i32) -> bool {
            let mut gen = PCGen::new(s,i);
            let (low, high) = match a.cmp(&b) {
                Ordering::Less => (a,b),
                Ordering::Equal => (a,b+1),
                Ordering::Greater => (b,a),
            };
            let result = gen.in_range(low, high);
            low <= result && result < high
        }
    }
}

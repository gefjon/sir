use std::iter::Iterator;

pub type SirNumeric = f64;
pub type SirInteger = isize;

fn delta_s(
    beta: SirNumeric,
    susceptible: SirNumeric,
    infected: SirNumeric,
    total_pop: SirNumeric,
) -> SirNumeric {
    -((beta * susceptible * infected) / total_pop)
}

fn delta_r(gamma: SirNumeric, infected: SirNumeric) -> SirNumeric {
    gamma * infected
}

fn delta_i(delta_s: SirNumeric, delta_r: SirNumeric) -> SirNumeric {
    (-delta_s) - delta_r
}

fn assert_sum_equal_enough(vals: &[SirNumeric], other: SirNumeric) {
    use std::f64::EPSILON;
    use std::iter::Sum;

    const EPS: SirNumeric = EPSILON;

    if vals.is_empty() {
        return;
    }

    let sum = f64::sum(vals.iter());

    let max = {
        let mut n = vals[0];
        for m in vals {
            n = n.max(*m);
        }
        n
    };

    assert!(
        ((sum - other).abs()) <= (max * EPS),
        "nums={:?} sum={} max={} compare={} diff={}",
        vals,
        sum,
        max,
        other,
        (sum - other).abs()
    );
}

#[derive(Clone, Debug)]
pub struct SirStep {
    pub day: SirInteger,
    pub susceptible: SirNumeric,
    pub infected: SirNumeric,
    pub removed: SirNumeric,
}

impl SirStep {
    fn advance(&self, beta: SirNumeric, gamma: SirNumeric, total_pop: SirNumeric) -> Self {
        let SirStep {
            day,
            susceptible,
            infected,
            removed,
        } = *self;

        let d_s = delta_s(beta, susceptible, infected, total_pop);
        let d_r = delta_r(gamma, infected);
        let d_i = delta_i(d_s, d_r);

        assert_sum_equal_enough(&[d_s, d_r, d_i], 0.0);

        SirStep {
            day: day + 1,
            susceptible: susceptible + d_s,
            infected: infected + d_i,
            removed: removed + d_r,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SirIterator {
    current: SirStep,
    number_of_days: SirInteger,
    beta: SirNumeric,
    gamma: SirNumeric,
    total_pop: SirNumeric,
}

impl SirIterator {
    pub fn from_total_pop(
        beta: SirNumeric,
        gamma: SirNumeric,
        total_pop: SirNumeric,
        infected: SirNumeric,
        number_of_days: SirInteger,
    ) -> Self {
        let susceptible = total_pop - infected;
        let removed = 0.0;

        let first_step = SirStep {
            day: 0,
            infected,
            susceptible,
            removed,
        };

        Self {
            current: first_step,
            number_of_days,
            beta,
            gamma,
            total_pop,
        }
    }

    #[allow(dead_code)]
    pub fn from_susceptible(
        beta: SirNumeric,
        gamma: SirNumeric,
        susceptible: SirNumeric,
        infected: SirNumeric,
        number_of_days: SirInteger,
    ) -> Self {
        let total_pop = susceptible + infected;
        let removed = 0.0;

        let first_step = SirStep {
            day: 0,
            infected,
            susceptible,
            removed,
        };

        Self {
            current: first_step,
            number_of_days,
            beta,
            gamma,
            total_pop,
        }
    }
}

impl Iterator for SirIterator {
    type Item = SirStep;
    fn next(&mut self) -> Option<SirStep> {
        if self.current.day >= self.number_of_days {
            return None;
        }
        let new = self.current.advance(self.beta, self.gamma, self.total_pop);
        self.current = new.clone();
        Some(new)
    }
}

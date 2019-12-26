/*!

--- Day 14: Space Stoichiometry ---

As you approach the rings of Saturn, your ship's low fuel indicator turns
on. There isn't any fuel here, but the rings have plenty of raw
material. Perhaps your ship's Inter-Stellar Refinery Union brand nanofactory
can turn these raw materials into fuel.

You ask the nanofactory to produce a list of the reactions it can perform that
are relevant to this process (your puzzle input). Every reaction turns some
quantities of specific input chemicals into some quantity of an output
chemical. Almost every chemical is produced by exactly one reaction; the only
exception, ORE, is the raw material input to the entire process and is not
produced by a reaction.

You just need to know how much ORE you'll need to collect before you can
produce one unit of FUEL.

Each reaction gives specific quantities for its inputs and output; reactions
cannot be partially run, so only whole integer multiples of these quantities
can be used. (It's okay to have leftover chemicals when you're done, though.)
For example, the reaction 1 A, 2 B, 3 C => 2 D means that exactly 2 units of
chemical D can be produced by consuming exactly 1 A, 2 B and 3 C. You can run
the full reaction as many times as necessary; for example, you could produce 10
D by consuming 5 A, 10 B, and 15 C.

Suppose your nanofactory produces the following list of reactions:

10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL

The first two reactions use only ORE as inputs; they indicate that you can
produce as much of chemical A as you want (in increments of 10 units, each 10
costing 10 ORE) and as much of chemical B as you want (each costing 1 ORE). To
produce 1 FUEL, a total of 31 ORE is required: 1 ORE to produce 1 B, then 30
more ORE to produce the 7 + 7 + 7 + 7 = 28 A (with 2 extra A wasted) required
in the reactions to convert the B into C, C into D, D into E, and finally E
into FUEL. (30 A is produced because its reaction requires that it is created
in increments of 10.)

Or, suppose you have the following list of reactions:

9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL

The above list of reactions requires 165 ORE to produce 1 FUEL:

Consume 45 ORE to produce 10 A.
Consume 64 ORE to produce 24 B.
Consume 56 ORE to produce 40 C.
Consume 6 A, 8 B to produce 2 AB.
Consume 15 B, 21 C to produce 3 BC.
Consume 16 C, 4 A to produce 4 CA.
Consume 2 AB, 3 BC, 4 CA to produce 1 FUEL.

Here are some larger examples:

13312 ORE for 1 FUEL:

157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT

180697 ORE for 1 FUEL:

2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF

2210736 ORE for 1 FUEL:

171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX

Given the list of reactions in your puzzle input, what is the minimum amount of
ORE required to produce exactly 1 FUEL?

--- Part Two ---

After collecting ORE for a while, you check your cargo hold: 1 trillion
(1000000000000) units of ORE.

With that much ore, given the examples above:

The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
The 2210736 ORE-per-FUEL example could produce 460664 FUEL.
Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?

Although it hasn't changed, you can still get your puzzle input.

 */

use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use crate::utils::{bail, insert_or_merge, ProblemInput, ProblemResult, SimpleError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Term {
    reagent: String,
    amount: u64,
}

impl FromStr for Term {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return bail(format!("Failed to parse term: {}", s));
        }
        let amount: u64 = parts[0]
            .parse()
            .map_err(|_e| SimpleError(format!("bad amount: {}", parts[0])))?;
        let reagent: String = parts[1].into();

        Ok(Term { reagent, amount })
    }
}

impl std::ops::Mul<u64> for Term {
    type Output = Self;

    fn mul(self, rhs: u64) -> Term {
        Term {
            reagent: self.reagent,
            amount: self.amount * rhs,
        }
    }
}

#[derive(Debug)]
struct Formula {
    inputs: Vec<Term>,
    output: Term,
}

impl FromStr for Formula {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" => ").collect();
        if parts.len() != 2 {
            return bail(&format!("Failed to parse formula: {}", s));
        }

        let output = Term::from_str(parts[1])?;
        let inputs: Result<Vec<Term>, Box<dyn Error>> =
            parts[0].split(", ").map(|s| Term::from_str(s)).collect();

        Ok(Formula {
            inputs: inputs?,
            output,
        })
    }
}

#[derive(Debug)]
struct Formulas {
    data: HashMap<String, Formula>,
}

impl FromStr for Formulas {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s
            .trim()
            .lines()
            .map(|l| match Formula::from_str(l) {
                Ok(formula) => {
                    let output = formula.output.reagent.clone();
                    Ok((output, formula))
                }
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<(String, Formula)>, Box<dyn Error>>>()?;

        let data: HashMap<String, Formula> = pairs.into_iter().collect();

        Ok(Formulas { data })
    }
}

impl Formulas {
    fn ore_for_fuel(&self, amount: u64) -> ProblemResult<u64> {
        produce(
            Term {
                reagent: "FUEL".into(),
                amount,
            },
            &self.data,
        )
    }
}

fn produce(target: Term, conversions: &HashMap<String, Formula>) -> ProblemResult<u64> {
    let mut leftovers = HashMap::new();
    produce_inner(target, conversions, &mut leftovers)
}

fn produce_inner(
    target: Term,
    conversions: &HashMap<String, Formula>,
    leftovers: &mut HashMap<String, u64>,
) -> ProblemResult<u64> {
    if target.reagent == "ORE" {
        return Ok(target.amount);
    }

    // If we have any leftovers for our current goal, use them first.
    let amount_to_produce = match leftovers.get_mut(&target.reagent) {
        Some(amount) => {
            let amount_consumed = min(*amount, target.amount);
            *amount -= amount_consumed;
            target.amount - amount_consumed
        }
        None => target.amount,
    };

    let formula = conversions
        .get(&*target.reagent)
        .ok_or(format!("No formula producingn reagent: {}", target.reagent))?;

    // How many times do we need to run the formula to produce the needed
    // amount of target?
    let iterations = {
        let mut tmp = amount_to_produce / formula.output.amount;
        if amount_to_produce % formula.output.amount > 0 {
            tmp += 1;
        }
        tmp
    };

    // Calculate how much ore we need to produce our inputs, the required
    // number of times.
    let mut total = 0;
    for input in formula.inputs.iter() {
        total += produce_inner(input.clone() * iterations, conversions, leftovers)?;
    }

    // Record any leftover output.
    let amount_leftover = iterations * formula.output.amount - amount_to_produce;
    if amount_leftover > 0 {
        insert_or_merge(leftovers, target.reagent, amount_leftover, |x, y| x + y);
    };

    Ok(total)
}

#[derive(Debug, Clone, Copy)]
enum ProbeType {
    Quadratic,
    Linear,
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    value: u64,
    probe_type: ProbeType,
}

impl Probe {
    fn get_value(&self) -> u64 {
        match self.probe_type {
            ProbeType::Quadratic => self.value * self.value,
            ProbeType::Linear => self.value,
        }
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}

pub fn run() -> ProblemResult<()> {
    let fs: Formulas = Formulas::for_problem(14)?;

    // Part 1
    println!("ORE needed to produce 1 FUEL: {}", fs.ore_for_fuel(1)?);

    // Part 2
    const MAX_ORE: u64 = 1_000_000_000_000;

    let mut guess = Probe {
        value: 1,
        probe_type: ProbeType::Quadratic,
    };
    let mut prev = guess;

    let fuel = loop {
        let produced = fs.ore_for_fuel(guess.get_value())?;
        if produced > MAX_ORE {
            match guess.probe_type {
                ProbeType::Quadratic => {
                    guess = Probe {value: prev.get_value(), probe_type: ProbeType::Linear};
                }
                ProbeType::Linear => {
                    break guess.get_value() - 1;
                }
            }
        }
        else {
            prev = guess;
            guess.increment();
        }
    };

    println!("Max fuel produced with {} ORE: {}", MAX_ORE, fuel);

    Ok(())
}

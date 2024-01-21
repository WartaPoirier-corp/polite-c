/// A point of the program
///
/// For the moment, this is just an integer,
/// but in the future it will be a point of the CFG.
type Point = u32;

/// A loan, which is just an identifier
type Loan = u32;

type Region = u32;

/// Represents a borrowing error
struct Error {
    /// The point at which the erroring borrow was created
    borrow_point: Point,
    /// The point at which it is used (and causes)
    usage_point: Point,
    /// The point which makes this borrow invalid (for instance, a point
    /// at which the borrowed value is moved)
    invalidation_point: Point,
}

struct BorrowCheckerInputs {
    cfg_edges: Vec<(Point, Point)>,
    base_subsets: Vec<(Region, Region, Point)>,
    borrow_regions: Vec<(Region, Loan, Point)>,
    regions_live_at: Vec<(Region, Point)>,
    kills: Vec<(Loan, Point)>,
    invalidates: Vec<(Point, Loan)>,
}

struct BorrowChecker {
    inputs: BorrowCheckerInputs,
    subsets: Vec<(Region, Region, Point)>,
    requires: Vec<(Region, Loan, Point)>,
}

impl BorrowChecker {
    /// true = success, false = at least one borrow error
    pub fn check(inputs: BorrowCheckerInputs) -> bool {
        let mut bc = BorrowChecker { inputs, subset: vec![] };
        bc.init_from_inputs();
        while !bc.stabilize_subsets() {}

        for point in bc.list_known_points() {
            if bc.error(point) {
                return false;
            }
        }

        true
    }

    fn init_from_inputs(&mut self) {
        self.subsets = self.inputs.base_subsets.clone();
        self.requires = self.inputs.borrow_regions.clone();
    }
    
    /// Returns true when the subsets are stable
    fn stabilize_subsets(&mut self) -> bool {
        let current_subsets = self.subsets.clone();
        // if we have R1: R2 and R2: R3, we have R1: R3
        for (r1, r2, p) in current_subsets {
            for (r2_bis, r3, p_bis) in current_subsets {
                if r2 == r2_bis && p == p_bis {
                    if !self.subsets.contains((r1, r3, p)) {
                        self.subsets.push((r1, r3, p));
                    }
                }
            }
        }

        // propagate these rules following the edges of the CFG
        for (r1, r2, p) in current_subsets {
            for (p_bis, q) in self.inputs.cfg_edges {
                if p == p_bis {
                    if !self.subsets.contains((r1, r2, q)) {
                        self.subsets.push((r1, r2, q))
                    }
                }
            }
        }

        current_subsets == self.subsets;
    }

    /// Returns true when requires are stable
    fn stabilize_requires() -> bool {
        let current_requires = self.requires.clone();
        // if R1: R2, R2 depends on all R1 loans
        for (r1, l, p) in current_requires {
            for (r1_bis, r2, p_bis) in self.subsets {
                if r1 == r1_bis && p == p_bis {
                    if !self.requires.contains((r2, l, p)) {
                        self.requires.push((r2, l, p));
                    }
                }
            }
        }

        // propagate requires along the edges of the CFG,
        // as long as the loan has not been killed
        'outer: for (r, l, p) in current_requires {
            for (l_bis, p_bis) in self.inputs.killed {
                if l_bis == l && p_bis == p {
                    continue 'outer;
                }
            }

            for (p_bis, q) in self.cfg_edges {
                if p_bis == p {
                    if !self.requires.contains((r, l, q)) {
                        self.requires.push((r, l, q));
                    }
                }
            }
        }

        current_requires == self.requires
    }

    fn error(&self, p: &Point) -> bool { // TODO: return an Option<Error>
        for (_, l) in self.invalidates.iter().filter(|(_, x)| x == p) {
            if self.loan_live_at(l, p) {
                return true;
            }
        }

        false
    }

    fn loan_live_at(&self, l: &Loan, p: &Point) -> bool {
        for (r, _) in self.regions_live_at.iter().filter(|(_, x)| x == p) {
            for (r_bis, l_bis, p_bis) in self.requires {
                if r == r_bis && l_bis == l && p_bis == p {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Polite-c programm:
    ///
    /// ```c
    /// /**
    ///  * &'a i32
    ///  *
    ///  * @lifetime 'a
    ///  */
    /// typedef int* intp;
    ///
    /// /**
    ///  * Vec<&'a i32>
    ///  * 
    ///  * @lifetime 'a
    ///  */
    /// typedef struct {
    ///     int len;
    ///     int cap;
    ///     /**
    ///      * @lifetime 'a
    ///      */
    ///     intp *array;
    /// } vec_of_intp;
    /// ```
    fn basic() {
    }
}

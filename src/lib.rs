//! # References
//! - Operators on Fuzzy Sets: Zadeh and Einstein (by Hannes Gassert.)

pub type Truth = f32;

/// Zadeh operators are best suited for logic operations that are exclusionary; membership in one
/// set implies a non-membership in another set.
pub mod zadeh {
    use ::Truth;

    /// Performs a Zadeh intersection of two truths. This is analogous to a boolean "or.".
    /// A and B are individual memberships of an item within classification A and classification B.
    pub fn min(a: Truth, b: Truth) -> Option<Truth> {
        if a.is_nan() || b.is_nan() { return None; }
        if a.is_infinite() || b.is_infinite() { return None; }

        if a > b {
            Some(b)
        } else {
            Some(a)
        }
    }

    #[inline(always)]
    /// Semantic sugar for the intersection operator.
    pub fn or(a: Truth, b: Truth) -> Option<Truth> {
        min(a, b)
    }

    /// Performs a Zadeh union of two truths. This is analogous to a boolean "and.".
    /// A and B are individual memberships of an item within classification A and classification B.
    pub fn max(a: Truth, b: Truth) -> Option<Truth> {
        if a.is_nan() || b.is_nan()           { return None; }
        if a.is_infinite() || b.is_infinite() { return None; }

        if a > b {
            Some(a)
        } else {
            Some(b)
        }
    }

    #[inline(always)]
    /// Semantic sugar for the union operator.
    pub fn and(a: Truth, b: Truth) -> Option<Truth> {
        max(a, b)
    }
}

/// Einstein operators are best suited when set membership is independent. Memebership in one set
/// has no implied bearing in membership in another set.
pub mod einstein {
    use ::Truth;

    /// Computes the Einstein product of two fuzzy set memberships.
    pub fn product(a: Truth, b: Truth) -> Option<Truth> {
        if a.is_nan() || b.is_nan()           { return None; }
        if a.is_infinite() || b.is_infinite() { return None; }

        let result = (a * b) / (1.0 + ((1.0 - a) * (1.0 - b)))
        ;
        if result.is_nan() || result.is_infinite() { return None; }
        Some(result)
    }

    #[inline(always)]
    /// Semantic sugar for the intersection operator.
    pub fn or(a: Truth, b: Truth) -> Option<Truth> {
        product(a, b)
    }

    /// Computes the Einstein sum of two fuzzy set memberships.
    pub fn sum(a: Truth, b: Truth) -> Option<Truth> {
        if a.is_nan() || b.is_nan()           { return None; }
        if a.is_infinite() || b.is_infinite() { return None; }

        let result = (a + b) / (1.0 + (a * b));

        if result.is_nan() || result.is_infinite() { return None; }
        Some(result)
    }

    #[inline(always)]
    /// Semantic sugar for the union operator.
    pub fn and(a: Truth, b: Truth) -> Option<Truth> {
        sum(a, b)
    }
}

pub mod werner {
    use ::Truth;

    /// Implement's Werner's "fuzzy and" operator, which functions as a type of
    /// "averaging" operator across fuzzy set memberships. Weight should be between zero and one.
    pub fn weighted_min(weight: f32, a: Truth, b: Truth) -> Option<Truth> {
        // NB we aren't enforcing the weight's domain; it might be worth doing?
        match ::zadeh::min(a, b) {
            Some(x) => {
                let result = ((weight * x) + ((1.0 - weight) * (a + b))) / 2.0;
                if result.is_nan() || result.is_infinite() {
                    None
                } else {
                    Some(result)
                }
            },
            None => None,
        }
    }

    #[inline(always)]
    /// Semantic sugar for the weighted min operator.
    pub fn fuzzy_and(weight: f32, a: Truth, b: Truth) -> Option<Truth> {
        weighted_min(weight, a, b)
    }
}

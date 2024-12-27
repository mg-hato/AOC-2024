
/// Zips two results using the zip function and returns a zipped result. Outcomes are as follows:
/// 1. If `left` is error, returns it.
/// 2. If `left` is ok and `right` is error, returns `right` error.
/// 3. `left` and `right` are `Ok(lhs)` and `Ok(rhs)`, respectively, then it returns `Ok(zip_func(lhs, rhs))`
pub fn zip<L, R, U, E, ZF>(left: Result<L, E>, right: Result<R, E>, zip_func: ZF) -> Result<U, E>
  where ZF: FnOnce(L, R) -> U {
    left.and_then(|l| right.map(|r| zip_func(l, r)))
}

/// Collects a vector of results into a result of a vector. It does that in the following way:
/// 1. If all results in the vector are ok, it extracts them in a vector `v` and returns `Ok(v)`.
/// 2. Otherwise, it returns a first error in the vector.
pub fn collect<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>,E> {
    let mut rtn = vec![];
    for result in results {
        match result {
            Ok(item) => rtn.push(item),
            Err(e) => return Err(e)
        }
    }
    Ok(rtn)
}
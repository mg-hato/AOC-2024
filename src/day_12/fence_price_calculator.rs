use std::collections::HashSet;

use crate::{answer::{Answer, DisplayableAnswer}, helper::{boundary::apply, movement, table::Table}, solver::Solve};

pub struct FencePriceCalculator;

mod error {
    const PREFIX: &str = "[Solver D-12]";

    pub fn overlflow(acc: u64, perimeter: u64, area: u64) -> String {
        format!("{} overflow occurred during price calculation step: {} + ({} * {})", PREFIX, acc, perimeter, area)
    }
}

impl FencePriceCalculator {

    fn region_pricing_step(price_acc: u64, new_region_perimeter: u64, new_region_area: u64) -> Result<u64, String> {
        new_region_perimeter.checked_mul(new_region_area)
            .and_then(|product|product.checked_add(price_acc))
            .ok_or_else(||error::overlflow(price_acc, new_region_perimeter, new_region_area))
    }

    fn price(map: Table<char>) -> Result<u64, String> {
        let mut total_price = Ok(0);

        // done - set of already processed and accounted nodes
        let mut done = HashSet::new();

        for (position, &region_plant_type) in map.iter() {

            // if already processed: skip
            if done.contains(&position) { continue; }

            let mut perimeter = 0;
            let mut area = 0;
            let mut queue = vec![position]; // contains only positions whose type of plant is `c`

            while !queue.is_empty() {
                let position = queue.pop().unwrap();
                if done.contains(&position) { continue; }

                let neighbours = movement::unit::all_partial().iter()
                    .filter_map(|&movement|apply(map.boundary(), movement, position))
                    .filter(|upos|map.get_pos(upos.pos()).is_some_and(|&plant_type|plant_type == region_plant_type))
                    .collect::<Vec<_>>();

                // account for perimeter & area contribution of the current position
                perimeter = perimeter + 4 - (neighbours.len() as u64);
                area += 1;
                done.insert(position);

                neighbours.into_iter().for_each(|p|{ queue.push(p); });
            }
            // Now that the region is finished, calculate fence price for it
            total_price =  total_price.and_then(|price|Self::region_pricing_step(price, perimeter, area));
        }
        total_price
    }
}

impl Solve<Table<char>> for FencePriceCalculator {
    fn solve(&self, input: Table<char>) -> Result<Answer, String> {
        Self::price(input).map(DisplayableAnswer::new)
    }
}
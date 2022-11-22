use crate::db::queries::all_back_blasts::BackBlastJsonData;

/// get avg pax per bd for a given list of back blasts passed in.
pub fn get_avg_pax_per_bd(back_blasts: &[BackBlastJsonData]) -> f64 {
    let pax_numbers = back_blasts.iter().map(|bb| bb.pax.len());
    let sum = pax_numbers.sum::<usize>() as u32;
    let avg_pax_per_bd =
        f64::from(sum) / f64::from(back_blasts.iter().map(|bb| bb.pax.len()).len() as u32);
    avg_pax_per_bd
}

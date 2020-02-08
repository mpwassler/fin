#[macro_use]
extern crate clap;

use clap::App;

fn main() {
	let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("early-payoff") {
        println!("welcome to early payoff");
        println!("Using input file: {}", matches.value_of("principal").unwrap());
        
    }

    if let Some(matches) = matches.subcommand_matches("interest") {

        let principal = matches.value_of("principal").unwrap();
        let interest = matches.value_of("interest").unwrap();
        let total_months = matches.value_of("total-months").unwrap();

        let principal_float : f64 = principal.parse().unwrap();
        let interest_float : f64 = interest.parse().unwrap();
        let total_months_float : i64 = total_months.parse().unwrap();

        let (total, monthly) = calculate_simple_interest(principal_float, interest_float, total_months_float);
        println!("Total Ammount: {}", total);
        println!("monthly Payments: {}", monthly);
        
    }
}


fn calculate_simple_interest(principal: f64, interest: f64, period: i64) -> (f64, f64) {
	
	let period_float = period as f64;

	let interest_per_month = interest / 12.00;

	let interest_per_month_decimal = interest_per_month / 100.00;

	let top = interest_per_month_decimal * (1.00 + interest_per_month_decimal).powf(period_float);
	
	let bottom = (1.00 + interest_per_month_decimal).powf(period_float) - 1.00;

	let fraction = top / bottom; 
	
	let monthly_payments = principal * fraction;

	((monthly_payments * period_float).round(),  monthly_payments.round())

}


#[cfg(test)]

mod tests {
	use super::*;

	#[test]
    fn test_calculate_simple_interest() {
        assert_eq!(calculate_simple_interest(25000.00, 3.11, 60), (27026.00, 450.00));
    }
}
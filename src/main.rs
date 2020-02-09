#[macro_use]
extern crate clap;
extern crate currency;

use clap::App;
use math::round;

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
	let monthly_payments = calultate_monthly_payment(principal, interest, period_float);
	((monthly_payments * period_float),  monthly_payments)

}

fn calultate_monthly_payment(principal: f64, interest: f64, period: f64) -> f64 {
	let interest_per_month = round::floor(interest / 12.00, 9);
	let interest_per_month_decimal = interest_per_month / 100.00;
	let top = interest_per_month_decimal * (1.00 + interest_per_month_decimal).powf(period);
	let bottom = (1.00 + interest_per_month_decimal).powf(period) - 1.00;
	let fraction = round::floor(top, 9) / round::floor(bottom, 9); 	
	round::floor(principal * fraction, 2)
}


#[cfg(test)]

mod tests {
	use super::*;

	#[test]
    fn test_calculate_simple_interest() {
    	let (total, monthly) = calculate_simple_interest(25000.00, 3.11, 60);
    	let int_total = total as i64;
    	let int_monthly = monthly as i64;
    	assert_eq!(int_total, 27026);
        assert_eq!(int_monthly, 450);
    }

    #[test]
    fn test_calculate_monthly_payment() {
    	let as_int = calultate_monthly_payment(25000.00, 3.11, 60.00) as i64;
    	assert_eq!(as_int, 450);	
    }
}

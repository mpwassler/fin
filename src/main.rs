#[macro_use]
extern crate clap;
extern crate currency;

mod loan;

use clap::App;
use math::round;
use loan::Loan;

fn main() {
	let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("early-payoff") {
        println!("welcome to early payoff");
        println!("Using input file: {}", matches.value_of("principal").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("interest") {        

        let loan = Loan::from_args(matches);

        let (total, monthly) = calculate_simple_interest(loan.principal, loan.interest, loan.terms);

        show_amoritization_table(loan.principal, loan.interest, monthly);
        
        println!("Total Ammount: {}", total);
        println!("monthly Payments: {}", monthly);
        
    }
}

fn show_amoritization_table(principal: f64, interest: f64, payment: f64) {
	let mut principal_remaining = principal;
	let mut payments_made = 1;

	let interest_per_month = calculate_monthly_interest(interest);
    	    
    println!(
        "{0: <20} | {1: <20} | {2: <20} | {3: <20}",
        "payment", "principal remaining", "principal paid", "interest paid"
    );

	while round::floor(principal_remaining, 0) > 0.0 {
		let months_interest = principal_remaining * interest_per_month;
		let payment_to_principal = payment - months_interest;

		principal_remaining -= payment_to_principal;
	    println!(
            "{0: <20} | {1: <20} | {2: <20} | {3: <20}",
            payments_made, round::half_away_from_zero(principal_remaining, 2), round::half_away_from_zero(payment_to_principal, 2), round::half_away_from_zero(months_interest, 2)
	    );

	    payments_made += 1;
	}			    
}


fn calculate_simple_interest(principal: f64, interest: f64, period: f64) -> (f64, f64) {
	let monthly_payments = calultate_monthly_payment(principal, interest, period);
	((monthly_payments * period),  monthly_payments)
}

fn calultate_monthly_payment(principal: f64, interest: f64, period: f64) -> f64 {
	let interest_per_month = calculate_monthly_interest(interest);
	let top = interest_per_month * (1.00 + interest_per_month).powf(period);
	let bottom = (1.00 + interest_per_month).powf(period) - 1.00;
	let fraction = round::floor(top, 9) / round::floor(bottom, 9); 	
	round::floor(principal * fraction, 2)
}

fn calculate_monthly_interest(interest: f64) -> f64 {
	let interest_per_month = interest / 12.00;
	round::half_away_from_zero(interest_per_month / 100.00, 15)
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

    #[test]
    fn test_calculate_monthly_interest() {    	    	
    	assert_eq!(calculate_monthly_interest(3.11), 0.002591666666667);
    }
}

use clap::{ArgMatches};

pub struct Loan {
	pub principal: f64, 
	pub interest: f64, 
	pub terms: f64
}

impl Loan {
	pub fn from_args(args: &clap::ArgMatches<'_>) -> Loan {
		let principal = args.value_of("principal").unwrap();
        let interest = args.value_of("interest").unwrap();
        let total_months = args.value_of("total-months").unwrap();        

        Loan {
        	principal: principal.parse().unwrap(),
        	interest: interest.parse().unwrap(),
        	terms: total_months.parse().unwrap()
        }
	}
}
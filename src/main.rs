use pars_kma::list_parser;

fn main() {
    let parsed_result = list_parser::list("[1,1,2,3,5,8]");
    assert_eq!(parsed_result, Ok(vec![1, 1, 2, 3, 5, 8]));
    println!("Parsed list: {:?}", parsed_result);
}

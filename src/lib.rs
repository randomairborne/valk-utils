use std::str::FromStr;

pub fn parse_var<T: FromStr>(name: &str) -> T {
    get_var(name)
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Could not parse {name} as {}!", std::any::type_name::<T>()))
}

pub fn get_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("Expected {name} in the environment!"))
}

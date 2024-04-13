use std::str::FromStr;

pub fn parse_var<T: FromStr>(name: &str) -> T {
    get_var(name)
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Could not parse {name} as {}!", std::any::type_name::<T>()))
}

pub fn parse_var_or<T: FromStr>(name: &str, value: T) -> T {
    parse_var_or_else(name, || value)
}

pub fn parse_var_or_else<T: FromStr>(name: &str, default: impl FnOnce() -> T) -> T {
    if let Ok(data) = std::env::var(name) {
        data.parse::<T>().unwrap_or_else(|_| default())
    } else {
        default()
    }
}

pub fn get_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("Expected {name} in the environment!"))
}

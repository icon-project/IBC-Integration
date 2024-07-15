#[macro_export]
macro_rules! cw_println {
   ($f:expr,$($arg:tt)*) => {{
        use cosmwasm_std::Api;
        let res = std::fmt::format(format_args!($($arg)*));
        debug_print::debug_println!("{}",res);
        $f.debug(&res);
    }};
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn test_print_macro() {
        let q = 10;
        let mut deps = mock_dependencies();
        cw_println!(deps.as_mut().api, "hello {}", q);
    }
}

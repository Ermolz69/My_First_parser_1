peg::parser! {
    pub grammar list_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {?
                n.parse().or(Err("Failed to parse u32"))
            }

        pub rule list() -> Vec<u32>
            = "[" numbers:(number() ** ",") "]" {
                numbers
            }
    }
}
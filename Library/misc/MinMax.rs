macro_rules! chmin {
    ($x:expr, $($v:expr),+) => {
        $(
            $x = std::cmp::min($x, $v);
        )+
    }
}

macro_rules! chmax {
    ($x:expr, $($v:expr),+) => {
        $(
            $x = std::cmp::max($x, $v);
        )+
    }
}

macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::min($x, min!( $($xs),+ ))
    };
}

macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::max($x, max!( $($xs),+ ))
    };
}
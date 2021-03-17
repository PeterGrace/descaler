use crate::lib::config::{ScalerConfig, ScalerResource};
use anyhow::{Result, Error};

macro_rules! assert_err {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
        }
    }
}


#[cfg(test)]
fn test_scalerconfig_invalid_yaml(){
    let test_string = String::from("@#%@#^@#^%^$%#^@#$@#$%^");
    let obj = ScalerConfig::from_string(test_string);
    assert_err!(obj,anyhow::Error);
}
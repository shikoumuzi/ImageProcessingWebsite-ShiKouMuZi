#[macro_export]
macro_rules! url {
      ( $( $x:expr )?) => {
        {
            let mut tmp_url = "".to_string();
            $(
                tmp_url += $x;
            )?
            tmp_url
        }
    };
}
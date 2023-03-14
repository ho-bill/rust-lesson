use std::env;

pub fn got_env_number( env_key: String, min_value: u32, max_value: u32, def_value: u32, no_err: usize ) -> u32 {
    let _out_value = def_value;
    if no_err == 0 {
        print!("讀取環境變數 {} ", env_key);
    }
    let env_str = match env::var_os(env_key) {
        Some(env_str) => env_str.into_string().unwrap(),
        None => {
            if no_err == 0 {
                print!("不存在, 改用預設值 ");
            }
            format!("{}", def_value)
        }
    };
    let _out_value: u32 = match env_str.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            if no_err == 0 {
                print!("發生錯誤，改用預設值 ");
            }
            def_value
        }
    };
    if _out_value >= min_value && _out_value <= max_value {
        if no_err == 0 {
            println!(": {}", _out_value);
        }
        _out_value
    } else {
        if no_err == 0 {
            println!(" 超過接受範圍 {} ~ {},  改用預設值{} ", min_value, max_value, def_value);
        }
        def_value
    }
}
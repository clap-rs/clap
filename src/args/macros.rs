
macro_rules! yaml_vec_or_str {
    ($v:ident, $a:ident, $c:ident) => {{
            let maybe_vec = $v.as_vec();
            if let Some(vec) = maybe_vec {
                for ys in vec {
                    if let Some(s) = ys.as_str() {
                        $a = $a.$c(s);
                    } else {
                        panic!("Failed to convert YAML value {:?} to a string", ys);
                    }
                }
            } else {
                if let Some(s) = $v.as_str() {
                    $a = $a.$c(s);
                } else {
                    panic!("Failed to convert YAML value {:?} to either a vec or string", $v);
                }
            }
            $a
        }
    };
}

macro_rules! yaml_to_str {
    ($a:ident, $v:ident, $c:ident) => {{
        $a.$c($v.as_str().unwrap_or_else(|| panic!("failed to convert YAML {:?} value to a string", $v)))
    }};
}

macro_rules! yaml_to_bool {
    ($a:ident, $v:ident, $c:ident) => {{
        $a.$c($v.as_bool().unwrap_or_else(|| panic!("failed to convert YAML {:?} value to a string", $v)))
    }};
}

macro_rules! yaml_to_u64 {
    ($a:ident, $v:ident, $c:ident) => {{
        $a.$c($v.as_i64().unwrap_or_else(|| panic!("failed to convert YAML {:?} value to a string", $v)) as u64)
    }};
}

macro_rules! yaml_to_usize {
    ($a:ident, $v:ident, $c:ident) => {{
        $a.$c($v.as_i64().unwrap_or_else(|| panic!("failed to convert YAML {:?} value to a string", $v)) as usize)
    }};
}

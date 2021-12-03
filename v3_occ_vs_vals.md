# Flags

-f?
-f*
-f{$0,3}: flags can't be required by default, hence always 0 or more (bounded/unbounded)

# Options

-o val
-o val?
-o val*
-o val+
-o val{0,3}

(-o val){0,3}
(-o val?){0,3}
(-o val*){0,3}
(-o val+){0,3}
(-o val{0,3}){0,3}

# A random user-agent

base a project `surferua (go-lang)` (git@github.com:jiusanzhou/surferua.git)  -- thank you

```rs

use crate::randua;

fn main(){
    randua::new().firefox().phone().to_string(); // a firefox and mobile user agent
    randua::new().chrome().phone().to_string(); // a chrome and mobile user agent
    randua::new().chrome().desktop().to_string(); // a chrome and desktop user agent
    randua::new().to_string(); // a random user agent
}

```

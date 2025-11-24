use hashbrown::HashMap;

use reportify::ResultExt;
use xscript::{read_str, run, Run};

use crate::system::boot_flows::BootFlowResult;

pub fn load_vars() -> BootFlowResult<HashMap<String, String>> {
    let mut env = HashMap::new();
    for line in read_str!(["fw_printenv"])
        .whatever("unable to read U-Boot environment")?
        .lines()
        .map(|l| l.trim())
    {
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        env.insert(key.to_owned(), value.to_owned());
    }
    Ok(env)
}

pub fn set_vars(vars: &HashMap<String, String>) -> BootFlowResult<()> {
    let mut script = String::new();
    for (key, value) in vars {
        script.push_str(key);
        script.push('=');
        script.push_str(value);
        script.push('\n');
    }
    run!(["fw_setenv", "-s", "-"].with_stdin(script))
        .whatever("unable to set U-Boot environment variables")?;
    Ok(())
}

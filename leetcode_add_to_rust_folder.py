from pathlib import Path
import re

complete_pattern = re.compile(r"""/*
* @lc app=leetcode id=\((?P<id1>)\) (?P<lang_rust_marker>lang=rust)
*
* \[(?P<id2>\d)\] Gas Station
*/

// (?P<lc_code_start>@lc code=start)
(?P<starting_impl_solution>impl Solution \{)
    pub fn (?P<fn_name>[A-z]+)\(.+\) -> .+ \{
    \}
(?P<ending_impl_solution>\})
// (?P<lc_code_end>@lc code=end)\s*
""")

def work_on_input_file(path:Path):
    print(complete_pattern)
    match = complete_pattern.match(path.read_text())
    print(match)

if __name__ == '__main__':
    work_on_input_file(Path("rust/134.gas-station.rs"))
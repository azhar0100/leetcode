from pathlib import Path
import re

# Fixed pattern - using re.DOTALL (or the inline flag (?s)) to match across lines
complete_pattern = re.compile(r"""(?s)/\*
 \* @lc app=leetcode id=(?P<id1>\d+) lang=rust
 \*
 \* \[(?P<id2>\d+)\] (?P<problem_name>Gas Station)
 \*/

// @lc code=start
impl Solution \{
    pub fn (?P<fn_name>[A-Za-z_]+)\((?P<args>.*?)\) -> (?P<return_type>.*?) \{
        
    \}
\}
// @lc code=end
""")

# name = 134.gas-station.rs
name_pattern = re.compile(r"(?P<id>\d+)\.(?P<problem_name>.+?)\.rs")

def work_on_input_file(path: Path, output_folder: Path):
    path_name = path.name
    content = path.read_text()
    match_content = complete_pattern.search(content)  # Using search instead of match
    id1 = match_content.group('id1')
    id2 = match_content.group('id2')
    assert id1 == id2
    fn_name = match_content.group('fn_name')
    args = match_content.group('args')
    return_type = match_content.group('return_type')
    problem_name = match_content.group('problem_name')
    match_name = name_pattern.search(path_name)
    id_name = match_name.group('id')
    problem_name_filename = match_name.group('problem_name')
    new_name = f"p{id_name.zfill(4)}_{problem_name_filename.replace("-","_")}.rs"
    print(f"{path_name} -> {new_name}")
    new_content = f"""
    pub fn {fn_name}({args}) -> {return_type} {{
        // {problem_name}
    }}
    """
    output_folder.joinpath(new_name).write_text(new_content)

    

if __name__ == '__main__':
    work_on_input_file(Path("rust/134.gas-station.rs"),Path("rust/leetcode-rust/src/problems"))
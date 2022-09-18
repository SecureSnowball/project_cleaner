# Build Files Cleaner
This is a very simple and dangerous project, It takes list of folder and give you list of projects in the following languages which are not clean (contains node_modules, vendor, etc);
- NodeJS
- PHP
- Rust

And you can clean those projects.
It does the following operations
- For NodeJS, it will remove node_modules folders
- For PHP it will remove vendor folder
- For rust it will run rust clean

### How to use
```bash
cargo run -- ~/Projects
# replace ~/Projects with the folder you want to scan
```

Sample output
```
╔══════════════════════════════════════════════════════════════════════╗
║                               Projects                               ║
╠═════╦════════════╦═════════╦═════════╦═══════════════════════════════╣
║  Sr ║ Language   ║ Has Git ║ Cleaned ║ Path                          ║
╠═════╬════════════╬═════════╬═════════╬═══════════════════════════════╣
║   0 ║ JavaScript ║ Yes     ║ No      ║ ~/Projects/project_one        ║
╠═════╬════════════╬═════════╬═════════╬═══════════════════════════════╣
║   1 ║ Rust       ║ Yes     ║ No      ║ ~/Projects/project_two        ║
╠═════╬════════════╬═════════╬═════════╬═══════════════════════════════╣
║   2 ║ PHP        ║ Yes     ║ No      ║ ~/Projects/project_three      ║
╠═════╬════════════╬═════════╬═════════╬═══════════════════════════════╣
║   3 ║ JavaScript ║ Yes     ║ No      ║ ~/Projects/project_four       ║
╠═════╬════════════╬═════════╬═════════╬═══════════════════════════════╣
║   4 ║ Rust       ║ Yes     ║ No      ║ ~/Projects/project_five       ║
╠═════╬════════════╬═════════╬═════════╬═══════════════════════════════╣
║   5 ║ JavaScript ║ Yes     ║ No      ║ ~/Projects/project_fix        ║
╚═════╩════════════╩═════════╩═════════╩═══════════════════════════════╝

Enter project number to clean.
- 1,2 to clean both 1 and 2 project
- a to clean all.
- q to quit
```

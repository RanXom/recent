# recent — TODO 
## Milestone 0: Define the contract 
- [x] Decide input format (directory path or current dir) 
- [x] Decide output shape (file path + modified time) 
- [x] Decide default behavior (non-recursive for now) 
___ 
## Milestone 1: Read file metadata (FOUNDATION) 
Goal: From a directory, list entries and print name + modified time. 
- [x] Read directory path from stdin 
- [x] Open directory using fs::read_dir 
- [x] Iterate over directory entries 
- [x] Extract PathBuf from each entry 
- [x] Read filesystem metadata for each path 
- [x] Filter out directories 
- [x] Extract last modified time (SystemTime) 
- [x] Print file_name + modified_time 
___ 
## Milestone 2: Walk the tree (RECURSION) 
Goal: Traverse directories inside directories. 
- [ ] Recurse into subdirectories 
- [ ] Preserve file metadata during recursion 
- [ ] Decide how to handle permission errors 
- [ ] Decide how to handle symlinks 
- [ ] Ensure no infinite loops 
___ 
## Milestone 3: Order reality (DATA → CONTROL) 
Goal: Treat filesystem info as data. 
- [ ] Collect (PathBuf, SystemTime) into a single vector 
- [ ] Stop printing during traversal 
- [ ] Sort by modified time (descending) 
- [ ] Verify deterministic output 
___ 
## Milestone 4: Make it humane (UX) 
Goal: Make output usable. 
- [ ] Limit output (top N) 
- [ ] Format timestamps for humans 
- [ ] Improve output layout 
- [ ] Graceful error reporting (no panics) 
___ 
## Milestone 5: Power-user features (OPTIONAL) 
- [ ] Recursive by default flag 
- [ ] --limit N 
- [ ] --days N 
- [ ] Ignore directories 
- [ ] Ignore hidden files 
- [ ] Support different time types (mtime / ctime) 
___ 
## Notes - Reuse logic before polishing - Do not optimize before correctness - Each milestone must leave the program runnable

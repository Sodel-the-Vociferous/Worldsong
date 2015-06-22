Rust source files in this directory are intended to define what processes should be called, and in what order, when the source file's execute() function is called (The boring details of this are hidden within the schedule! macro).

It's intended they be used for timing (fixed_update, variable_update, etc), but they could be used to define events as well (on_collide, on_mouseover, etc). The framework author just prefers the former.
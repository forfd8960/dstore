## Can't build the proto to rs

It's because the `build.rs` and the `dstore.proto` are inside of directory `src`, should move them outof src dir.

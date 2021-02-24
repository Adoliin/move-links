#!/usr/bin/bash

cwd="$(pwd)"

rm -rf test-dir
mkdir test-dir
mkdir test-dir/dir1 \
      test-dir/dir2 \
      test-dir/dir3 \
      test-dir/dir4 \
      test-dir/dir4/dir5 

touch test-dir/f1      \
      test-dir/dir2/f2 \
      test-dir/dir3/f3
      test-dir/dir4/f4

ln -s "$cwd/test-dir/f1" "$cwd/test-dir/dir1/f1-l1"
ln -s "$cwd/test-dir/f1" "$cwd/test-dir/dir4/dir5/f1-l2"
ln -s "$cwd/test-dir/dir2/f2" "$cwd/test-dir/dir4/f2-l1"
ln -s "$cwd/test-dir/dir2/f2" "$cwd/test-dir/f2-l1"
ln -s "$cwd/test-dir/dir4/dir5" "$cwd/test-dir"
ln -s "$cwd/test-dir/dir3" "$cwd/test-dir/dir2"

#Tree respresentation:
#
#test-dir
#│
#├── dir1
#│   └── f1-l1 -> $cwd/test-dir/f1
#├── dir1
#│   └── f1-l1 -> $cwd/test-dir/f1
#├── dir2
#│   ├── dir3 -> $cwd/test-dir/dir3
#│   └── f2
#├── dir3
#│   └── f3
#├── dir4
#│   ├── dir5
#│   │   └── f1-l2 -> $cwd/move-links/test-dir/f1
#│   └── f2-l1 -> $cwd/move-links/test-dir/dir2/f2
#├── dir5 -> $cwd/move-links/test-dir/dir4/dir5
#├── f1
#└── f2 -> $cwd/move-links/test-dir/dir2/f2

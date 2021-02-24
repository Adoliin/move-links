#!/usr/bin/env sh

cwd="$(pwd)"

rm -rf test-dir
mkdir "$cwd/test-dir"      \
      "$cwd/test-dir/dir1" \
      "$cwd/test-dir/dir2" \
      "$cwd/test-dir/dir3" \
      "$cwd/test-dir/dir4" \
      "$cwd/test-dir/dir4/dir5"

touch "$cwd/test-dir/f1"      \
      "$cwd/test-dir/dir2/f2" \
      "$cwd/test-dir/dir3/f3" \
      "$cwd/test-dir/dir4/f4"

ln -s "$cwd/test-dir/f1"        "$cwd/test-dir/dir1/f1-l1"
ln -s "$cwd/test-dir/f1"        "$cwd/test-dir/dir4/dir5/f1-l2"

ln -s "$cwd/test-dir/dir2/f2"   "$cwd/test-dir/dir4/f2-l1"
ln -s "$cwd/test-dir/dir2/f2"   "$cwd/test-dir/dir2/f2-l2"
ln -s "$cwd/test-dir/dir2/f2"   "$cwd/test-dir/f2-l3"

ln -s "$cwd/test-dir/dir4/dir5" "$cwd/test-dir/dir5-l1"
ln -s "$cwd/test-dir/dir4/dir5" "$cwd/test-dir/dir4/dir5-l2"
ln -s "$cwd/test-dir/dir4/dir5" "$cwd/test-dir/dir5-l3"

ln -s "$cwd/test-dir/dir3"      "$cwd/test-dir/dir1/dir3"
ln -s "$cwd/test-dir/dir3"      "$cwd/test-dir/dir2/dir3"
ln -s "$cwd/test-dir/dir3"      "$cwd/test-dir/dir3/dir3"

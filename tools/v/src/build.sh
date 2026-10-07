#!/bin/sh
# Builds V from source at the tag $1 in the working directory, the install dir that
# `native_install` runs this in, logging to stderr. Prints the sha256 that pins the build
# to its sources: that of the line `<v commit> <vc commit>`.
#
# The install dir is V's repository checked out at the tag, as V expects to run from its
# tree. V bootstraps from the vlang/vc snapshot of the newest v commit the tag contains,
# and `make local=1` builds it with the system cc without fetching vc or tcc.
set -eu
exec 3>&1 1>&2
tag=$1

# Idempotent, as a forced reinstall runs this in the tree of the earlier build.
git init -q
git config remote.origin.url https://github.com/vlang/v
# tree:0 fetches the tag's history as commits only, enough to tell which vc snapshot it contains.
git fetch -q --filter=tree:0 --no-tags origin "+refs/tags/$tag:refs/tags/$tag"
git checkout -q -f "$tag"

[ -d vc ] || git clone -q --filter=tree:0 --no-checkout https://github.com/vlang/vc vc
# vc's commit subjects name the v commit of their snapshot as `[v:master] <sha>`, the sha
# abbreviated to 7 characters in older ones; git log lists the newest first.
vc=$({ git rev-list HEAD; echo --; git -C vc log --format='%H %s'; } | awk '
    $0 == "--" { snapshots = 1; next }
    !snapshots { full[$1]; short[substr($1, 1, 7)]; next }
    $2 == "[v:master]" && ($3 in full || $3 in short) { print $1; exit }
')
if [ -z "$vc" ]; then
    echo "no vlang/vc snapshot of a v commit in $tag" >&2
    exit 1
fi
git -C vc checkout -q "$vc"

# Non-prod builds with cc link the Boehm GC from tcc's bundle, which this build skips, so
# build that archive from V's own bdwgc amalgamation, with the defines that
# vlib/builtin/builtin_d_gcboehm.c.v compiles it with.
gc_flags=
[ "$(uname -s)" = Darwin ] && gc_flags='-DLARGE_CONFIG=1 -DMPROTECT_VDB=1'
mkdir -p thirdparty/tcc/lib
# shellcheck disable=SC2086 # gc_flags holds several flags
cc -O2 -w -fPIC -DGC_BUILTIN_ATOMIC=1 -DGC_THREADS=1 -DTHREAD_LOCAL_ALLOC=1 \
    -DALL_INTERIOR_POINTERS=1 $gc_flags -Ithirdparty/libgc/include \
    -c thirdparty/libgc/gc.c -o thirdparty/tcc/lib/gc.o
ar rcs thirdparty/tcc/lib/libgc.a thirdparty/tcc/lib/gc.o

# skip_fastc leaves out the FastC backend, which links tcc's libtcc.a. -new-compiler keeps
# a failed C compile from downloading V 0.5.2 to retry with.
make local=1 VFLAGS='-new-compiler -d skip_fastc'

printf '%s %s\n' "$(git rev-parse HEAD)" "$vc" | { sha256sum 2>/dev/null || shasum -a 256; } | cut -d' ' -f1 >&3

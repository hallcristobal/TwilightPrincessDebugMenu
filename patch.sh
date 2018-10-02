rm -rf patch/ 2>/dev/null
mkdir patch 2>/dev/null
romhack build -p
cp target/tpgz.patch patch/$1.patch
cp USAGE.md patch/
cp CHANGELOG.md patch/

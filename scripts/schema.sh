# Core
echo "> core"
cargo run --manifest-path ./core/Cargo.toml schema;

# Demo contracts
for dir in ./example/contracts/*/
do
  dir="${dir%*/}"
  dirname="${dir##*/}"
  if [ "$dirname" != shared ]; then
    echo "\n> ${dirname}";
    cargo run --manifest-path ${dir}/Cargo.toml schema;
  fi
done

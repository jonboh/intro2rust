crop() {
    local filename=$1
    magick "$filename" -gravity South \
        -background white -splice 0x1  -background black -splice 0x1 \
        -trim  +repage -chop 0x1 "$filename"
    magick "$filename" -gravity North \
        -background white -splice 0x1  -background black -splice 0x1 \
        -trim  +repage -chop 0x1 "$filename"
    magick "$filename" -gravity South -chop 0x101 "$filename"

}

screencapture() {
    local project_name=$1
    echo '#$ expect \$'>> commands.sh
    mkdir -p build && asciinema-automation -dt 1 -wt 1 commands.sh build/"$project_name".cast; agg --font-size 25 --cols 120 --rows 40 --theme asciinema build/"$project_name".cast build/"$project_name".gif
    convert -coalesce build/"$project_name".gif build/out.png \
    && mv build/out-$(identify -format "%n\n" build/"$project_name".gif | head -n 1 | awk '{print $1-1}').png build/"$project_name".png
    crop build/"$project_name".png
    mkdir -p public/assets
    mv build/"$project_name".png public/assets/
}

asciinema_init() {
    echo '#$ wait 1000' > commands.sh
}

build_project() {
    local project_name=$1
    echo "build_project $project_name"
    asciinema_init
    echo "cd public/code/$project_name" >> commands.sh
    echo 'clear
cargo build' >> commands.sh
    screencapture $project_name
}

rustup_targets() {
    local system=$1
    echo "rustup_targets $system"
    asciinema_init
    echo "clear
rustup target list | rg $system" >> commands.sh
    screencapture "rustup_$system"
}

rustup_targets_count() {
    echo "rustup_targets_count"
    asciinema_init
    echo "clear
rustup target list | wc -l" >> commands.sh
    screencapture "rustup_targets_count"
}

rustup_install() {
    echo "rustup_install"
    asciinema_init
    echo "rustup toolchain remove stable-x86_64-unknown-linux-gnu
clear
rustup toolchain install stable-x86_64-unknown-linux-gnu" >> commands.sh
    screencapture "rustup_install"
}

cargo_init() {
    echo "cargo_init"
    asciinema_init
    echo "cd public/code
command rm -rf hello
mkdir hello
cd hello
clear
cargo init
tree --filesfirst
cat src/main.rs" >> commands.sh
    screencapture "cargo_init"
}
cargo_run() {
    echo "cargo_run"
    asciinema_init
    echo "cd public/code
command rm -rf hello
mkdir hello
cd hello
cargo init
clear
cargo build
cargo run
./target/debug/hello" >> commands.sh
    screencapture "cargo_run"
}

cargo_deps() {
    echo "cargo_deps"
    asciinema_init
    echo "cd public/code
command rm -rf hello
mkdir hello
cd hello
cargo init
clear
cargo add serde
cat Cargo.toml" >> commands.sh
    screencapture "cargo_deps"
}

cargo_project_structure() {
    echo "cargo_project_structure"
    asciinema_init
    echo "cd public/code/project_structure
clear
tree --filesfirst" >> commands.sh
    screencapture "cargo_project_structure"
}



command rm -rf build
# build_project "ownership"
# build_project "ownership_multiple"
# build_project "immutability"
# build_project "pattern_matching"
# rustup_targets "linux"
# rustup_targets "apple"
# rustup_targets "windows"
# rustup_targets "none"
# rustup_targets_count
# rustup_install
# cargo_init
# cargo_run
# cargo_deps
cargo_project_structure

rm commands.sh
rm build/out-*.png

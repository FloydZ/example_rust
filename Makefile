release:
	cargo build --release

build_timings:
	cargo build -Ztimings

# more informations can be found here: https://github.com/RazrFalcon/cargo-bloat
bloat:
	cargo bloat --release -n 10

bloat_crates:
	cargo bloat --release --crates

# more informations can be found here: https://github.com/psinghal20/cargo-feature-analyst
feature_analyse:
	cargo feature-analyst

# more informations can be found here: https://rustwasm.github.io/twiggy/
# Twiggy is a code size profiler for Wasm. It analyzes a binary's call graph to answer questions like:
#    Why was this function included in the binary in the first place? Who calls it?
#    What is the retained size of this function? I.e. how much space would be saved if I removed it and all the functions that become dead code after its removal.
# Use Twiggy to make your binaries slim!
twiggy:
	twiggy top
	twiggy paths
tree:
	cargo tree

watch:
	cargo watch

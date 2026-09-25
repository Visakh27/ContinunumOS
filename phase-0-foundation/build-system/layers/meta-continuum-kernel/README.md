# meta-continuum-kernel

Yocto recipe layer wrapping `../../../kernel/` — pins the LTS version
(`../../../kernel/VERSION_PIN`), applies the patch series
(`../../../kernel/patches/series`), and merges the Kconfig fragments in
`../../../kernel/config/`. Recipe implementation deferred (see parent README).
